use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use serde::Serialize;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::ops::codex::{
    CodexMode, CodexOptions, CodexRun, CodexSession, run_codex, run_codex_with_timeout,
};
use crate::ops::fs::{ensure_dir, write_file_atomic};
use crate::ops::loop_config::{LoopConfig, load_loop_config};
use crate::ops::loop_status::{
    LOOPR_STATUS_END, LOOPR_STATUS_START, LooprStatus, parse_loopr_status_from_log,
};
use crate::ops::loopr_root::resolve_loopr_root;
use crate::ops::run::{RunStep, build_prompt_lines, default_run_steps, find_step};
use crate::ops::work_plan::{TaskSpec, TestSpec, load_task_order, load_test_order};
use crate::ops::work_status::{
    TestRunResult, WorkItemState, WorkItemType, WorkStatusFile, ensure_item, load_work_status,
    write_work_status,
};
use crate::{LooprError, LooprResult};

pub struct LoopOptions {
    pub loopr_root: Option<PathBuf>,
    pub max_iterations: i64,
    pub per_task: bool,
    pub codex_args: Vec<String>,
    pub progress: Option<Box<dyn Fn(LoopEvent)>>,
}

pub struct LoopReport {
    pub iterations: i64,
    pub exit_reason: String,
    pub last_session: Option<CodexSession>,
}

pub struct LoopEvent {
    pub iteration: i64,
    pub status: String,
    pub details: String,
}

pub const LOOP_EVENT_START: &str = "start";
pub const LOOP_EVENT_DONE: &str = "done";
pub const LOOP_EVENT_EXIT: &str = "exit";
pub const LOOP_EVENT_ERROR: &str = "error";

struct LoopState {
    iteration: i64,
    missing_status_count: i64,
}

#[derive(Serialize)]
struct LoopStatusPayload {
    state: String,
    iteration: i64,
    updated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    exit_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_error: Option<String>,
}

pub fn run_loop(opts: LoopOptions) -> LooprResult<LoopReport> {
    let cwd = std::env::current_dir()?;
    let (root, _) = resolve_loopr_root(&cwd, opts.loopr_root.as_deref())?;
    let handoff_path = ensure_handoff(&root)?;
    let step = find_step(&default_run_steps(), "execute")
        .ok_or_else(|| LooprError::new("execute step not found"))?;

    let loopr_dir = root.join("loopr");
    let loopr_state_dir = loopr_dir.join("state");
    let config_path = loopr_dir.join("config");
    let status_path = loopr_state_dir.join("status.json");

    let mut cfg = load_loop_config(&config_path)?;
    if opts.max_iterations > 0 {
        cfg.max_iterations = opts.max_iterations;
    }

    if opts.per_task {
        return run_loop_per_task(
            &opts,
            root,
            cfg,
            handoff_path,
            status_path,
            loopr_state_dir,
            step,
        );
    }

    let mut state = LoopState {
        iteration: 0,
        missing_status_count: 0,
    };
    let mut report = LoopReport {
        iterations: 0,
        exit_reason: String::new(),
        last_session: None,
    };

    loop {
        if cfg.max_iterations > 0 && state.iteration >= cfg.max_iterations {
            report.exit_reason = "max_iterations".to_string();
            write_loop_status(
                &status_path,
                LoopStatusPayload {
                    state: "complete".to_string(),
                    iteration: state.iteration,
                    updated_at: now_rfc3339()?,
                    exit_reason: Some(report.exit_reason.clone()),
                    last_summary: None,
                    last_error: None,
                },
            )?;
            break;
        }

        let next_iteration = state.iteration + 1;
        if let Some(progress) = &opts.progress {
            progress(LoopEvent {
                iteration: next_iteration,
                status: LOOP_EVENT_START.to_string(),
                details: String::new(),
            });
        }

        let prompt = build_loop_prompt(&step, &handoff_path, next_iteration);
        let mut args = vec!["--cd".to_string(), root.display().to_string()];
        args.extend(opts.codex_args.clone());
        args.push(prompt);

        let run = if cfg.codex_timeout_minutes > 0 {
            run_codex_with_timeout(
                &args,
                &CodexOptions {
                    loopr_root: Some(root.clone()),
                    mode: CodexMode::Exec,
                },
                Duration::from_secs((cfg.codex_timeout_minutes as u64) * 60),
            )?
        } else {
            run_codex(
                &args,
                &CodexOptions {
                    loopr_root: Some(root.clone()),
                    mode: CodexMode::Exec,
                },
            )?
        };

        let mut run_error = codex_error(&run);
        report.last_session = Some(run.session);
        state.iteration = next_iteration;

        let mut status = LooprStatus::default();
        let mut status_found = false;
        if let Some(session) = &report.last_session {
            match parse_loopr_status_from_log(&session.log_path) {
                Ok((parsed, found)) => {
                    status = parsed;
                    status_found = found;
                }
                Err(err) => {
                    if run_error.is_none() {
                        run_error = Some(err);
                    }
                }
            }
        }

        if let Some(err) = &run_error {
            if status.summary.is_empty() {
                status.summary = err.message.clone();
            }
            status.status = "ERROR".to_string();
            status.exit_signal = false;
        }

        let (exit_reason, exit_state) = evaluate_loop_exit(&cfg, &status, status_found, &mut state);
        let payload = LoopStatusPayload {
            state: exit_state,
            iteration: state.iteration,
            updated_at: now_rfc3339()?,
            exit_reason: if exit_reason.is_empty() {
                None
            } else {
                Some(exit_reason.clone())
            },
            last_summary: if status.summary.is_empty() {
                None
            } else {
                Some(status.summary.clone())
            },
            last_error: if let Some(err) = &run_error {
                Some(err.message.clone())
            } else if !status_found {
                Some("missing LOOPR_STATUS block".to_string())
            } else {
                None
            },
        };

        write_loop_status(&status_path, payload)?;

        if !exit_reason.is_empty() {
            report.exit_reason = exit_reason.clone();
            if let Some(progress) = &opts.progress {
                progress(LoopEvent {
                    iteration: state.iteration,
                    status: LOOP_EVENT_EXIT.to_string(),
                    details: exit_reason.clone(),
                });
            }
            break;
        }

        if let Some(err) = run_error {
            if let Some(progress) = &opts.progress {
                progress(LoopEvent {
                    iteration: state.iteration,
                    status: LOOP_EVENT_ERROR.to_string(),
                    details: err.message.clone(),
                });
            }
            return Err(err);
        }

        if let Some(progress) = &opts.progress {
            progress(LoopEvent {
                iteration: state.iteration,
                status: LOOP_EVENT_DONE.to_string(),
                details: String::new(),
            });
        }
    }

    report.iterations = state.iteration;
    Ok(report)
}

fn run_loop_per_task(
    opts: &LoopOptions,
    root: PathBuf,
    cfg: LoopConfig,
    handoff_path: PathBuf,
    status_path: PathBuf,
    loopr_state_dir: PathBuf,
    step: RunStep,
) -> LooprResult<LoopReport> {
    let task_order_path = root.join("specs").join("task-order.yaml");
    let test_order_path = root.join("specs").join("test-order.yaml");
    let task_order = load_task_order(&task_order_path)?;
    let test_order = load_test_order(&test_order_path)?;

    let mut tasks = task_order.tasks;
    let mut tests = test_order.tests;
    tasks.sort_by_key(|task| task.id);
    tests.sort_by_key(|test| test.id);

    let mut task_by_id = HashMap::new();
    for task in &tasks {
        task_by_id.insert(task.id, task.clone());
    }
    let mut test_by_id = HashMap::new();
    for test in &tests {
        test_by_id.insert(test.id, test.clone());
    }
    let mut tests_by_task: HashMap<i64, Vec<TestSpec>> = HashMap::new();
    for test in &tests {
        tests_by_task.entry(test.task_id).or_default().push(test.clone());
    }
    for tests in tests_by_task.values_mut() {
        tests.sort_by_key(|test| test.id);
    }

    let work_status_path = loopr_state_dir.join("work-status.json");
    let mut work_status = load_work_status(&work_status_path, &now_rfc3339()?)?;
    let now = now_rfc3339()?;
    for task in &tasks {
        ensure_item(&mut work_status, &task.key, WorkItemType::Task, &now);
    }
    for test in &tests {
        ensure_item(&mut work_status, &test.key, WorkItemType::Test, &now);
        let is_pbt = detect_pbt(test, &root)?;
        if let Some(item) = work_status.items.get_mut(&test.key) {
            item.pbt = is_pbt;
            if item.state == WorkItemState::Complete && !item.tests_written {
                item.tests_written = true;
            }
        }
    }
    work_status.updated_at = now;
    write_work_status(&work_status_path, &work_status)?;

    let mut state = LoopState {
        iteration: 0,
        missing_status_count: 0,
    };
    let mut report = LoopReport {
        iterations: 0,
        exit_reason: String::new(),
        last_session: None,
    };

    loop {
        if cfg.max_iterations > 0 && state.iteration >= cfg.max_iterations {
            report.exit_reason = "max_iterations".to_string();
            write_loop_status(
                &status_path,
                LoopStatusPayload {
                    state: "complete".to_string(),
                    iteration: state.iteration,
                    updated_at: now_rfc3339()?,
                    exit_reason: Some(report.exit_reason.clone()),
                    last_summary: None,
                    last_error: None,
                },
            )?;
            break;
        }

        if let Some(reason) = blocked_reason(&work_status) {
            report.exit_reason = reason.clone();
            write_loop_status(
                &status_path,
                LoopStatusPayload {
                    state: if reason == "error" { "error" } else { "blocked" }.to_string(),
                    iteration: state.iteration,
                    updated_at: now_rfc3339()?,
                    exit_reason: Some(reason.clone()),
                    last_summary: None,
                    last_error: None,
                },
            )?;
            break;
        }

        if all_tasks_complete(&tasks, &work_status)
            && all_tests_validated(&tests, &work_status)
        {
            report.exit_reason = "completed".to_string();
            write_loop_status(
                &status_path,
                LoopStatusPayload {
                    state: "complete".to_string(),
                    iteration: state.iteration,
                    updated_at: now_rfc3339()?,
                    exit_reason: Some(report.exit_reason.clone()),
                    last_summary: None,
                    last_error: None,
                },
            )?;
            break;
        }

        let selection = select_next_item(
            &tasks,
            &tests,
            &task_by_id,
            &test_by_id,
            &tests_by_task,
            &work_status,
        );
        let selection = match selection {
            Some(value) => value,
            None => {
                report.exit_reason = "blocked".to_string();
                write_loop_status(
                    &status_path,
                    LoopStatusPayload {
                        state: "blocked".to_string(),
                        iteration: state.iteration,
                        updated_at: now_rfc3339()?,
                        exit_reason: Some(report.exit_reason.clone()),
                        last_summary: None,
                        last_error: Some("no runnable tasks/tests".to_string()),
                    },
                )?;
                break;
            }
        };

        let next_iteration = state.iteration + 1;
        let details = selection.describe();
        if let Some(progress) = &opts.progress {
            progress(LoopEvent {
                iteration: next_iteration,
                status: LOOP_EVENT_START.to_string(),
                details,
            });
        }

        let item_key = selection.key().to_string();
        let item_type = selection.item_type();
        let phase = selection.phase().to_string();
        update_item_in_progress(&mut work_status, &item_key, &phase, &now_rfc3339()?);
        write_work_status(&work_status_path, &work_status)?;

        let prompt_inputs = selection.prompt_inputs(&step);
        let prompt = build_per_task_prompt(
            &step,
            &handoff_path,
            next_iteration,
            selection.key(),
            item_type,
            &phase,
            &prompt_inputs,
            selection.is_pbt(),
        );

        let run = run_codex_for_prompt(&root, &cfg, &opts.codex_args, prompt)?;
        let mut run_error = codex_error(&run);
        report.last_session = Some(run.session);
        state.iteration = next_iteration;

        let mut status = LooprStatus::default();
        let mut status_found = false;
        if let Some(session) = &report.last_session {
            match parse_loopr_status_from_log(&session.log_path) {
                Ok((parsed, found)) => {
                    status = parsed;
                    status_found = found;
                }
                Err(err) => {
                    if run_error.is_none() {
                        run_error = Some(err);
                    }
                }
            }
        }

        if let Some(err) = &run_error {
            if status.summary.is_empty() {
                status.summary = err.message.clone();
            }
            status.status = "ERROR".to_string();
            status.exit_signal = false;
        }

        if status_found {
            state.missing_status_count = 0;
        } else {
            state.missing_status_count += 1;
            if state.missing_status_count >= cfg.max_missing_status {
                mark_item_failed(
                    &mut work_status,
                    &item_key,
                    WorkItemState::Error,
                    "missing LOOPR_STATUS block",
                    &now_rfc3339()?,
                );
                write_work_status(&work_status_path, &work_status)?;
                report.exit_reason = "missing_status".to_string();
                write_loop_status(
                    &status_path,
                    LoopStatusPayload {
                        state: "error".to_string(),
                        iteration: state.iteration,
                        updated_at: now_rfc3339()?,
                        exit_reason: Some(report.exit_reason.clone()),
                        last_summary: None,
                        last_error: Some("missing LOOPR_STATUS block".to_string()),
                    },
                )?;
                if let Some(progress) = &opts.progress {
                    progress(LoopEvent {
                        iteration: state.iteration,
                        status: LOOP_EVENT_ERROR.to_string(),
                        details: "missing LOOPR_STATUS block".to_string(),
                    });
                }
                break;
            }
        }

        if let Some(err) = run_error {
            mark_item_failed(
                &mut work_status,
                &item_key,
                WorkItemState::Error,
                &err.message,
                &now_rfc3339()?,
            );
            write_work_status(&work_status_path, &work_status)?;
            write_loop_status(
                &status_path,
                LoopStatusPayload {
                    state: "error".to_string(),
                    iteration: state.iteration,
                    updated_at: now_rfc3339()?,
                    exit_reason: Some("error".to_string()),
                    last_summary: Some(err.message.clone()),
                    last_error: Some(err.message.clone()),
                },
            )?;
            if let Some(progress) = &opts.progress {
                progress(LoopEvent {
                    iteration: state.iteration,
                    status: LOOP_EVENT_ERROR.to_string(),
                    details: err.message.clone(),
                });
            }
            break;
        }

        if status.status == "BLOCKED" {
            mark_item_failed(
                &mut work_status,
                &item_key,
                WorkItemState::Blocked,
                if status.summary.is_empty() {
                    "blocked by status"
                } else {
                    status.summary.as_str()
                },
                &now_rfc3339()?,
            );
            write_work_status(&work_status_path, &work_status)?;
            report.exit_reason = "blocked".to_string();
            write_loop_status(
                &status_path,
                LoopStatusPayload {
                    state: "blocked".to_string(),
                    iteration: state.iteration,
                    updated_at: now_rfc3339()?,
                    exit_reason: Some(report.exit_reason.clone()),
                    last_summary: Some(status.summary.clone()),
                    last_error: None,
                },
            )?;
            if let Some(progress) = &opts.progress {
                progress(LoopEvent {
                    iteration: state.iteration,
                    status: LOOP_EVENT_EXIT.to_string(),
                    details: "blocked".to_string(),
                });
            }
            break;
        }

        if status.status == "ERROR" {
            mark_item_failed(
                &mut work_status,
                &item_key,
                WorkItemState::Error,
                if status.summary.is_empty() {
                    "error status"
                } else {
                    status.summary.as_str()
                },
                &now_rfc3339()?,
            );
            write_work_status(&work_status_path, &work_status)?;
            report.exit_reason = "error".to_string();
            write_loop_status(
                &status_path,
                LoopStatusPayload {
                    state: "error".to_string(),
                    iteration: state.iteration,
                    updated_at: now_rfc3339()?,
                    exit_reason: Some(report.exit_reason.clone()),
                    last_summary: Some(status.summary.clone()),
                    last_error: None,
                },
            )?;
            if let Some(progress) = &opts.progress {
                progress(LoopEvent {
                    iteration: state.iteration,
                    status: LOOP_EVENT_ERROR.to_string(),
                    details: "error".to_string(),
                });
            }
            break;
        }

        match selection {
            WorkItemSelection::Test { test, task, .. } => {
                let test_outcome = match run_test_command(&root, &cfg.test_command, "tests") {
                    Ok(value) => value,
                    Err(err) => {
                        mark_item_failed(
                            &mut work_status,
                            &item_key,
                            WorkItemState::Error,
                            &err.message,
                            &now_rfc3339()?,
                        );
                        write_work_status(&work_status_path, &work_status)?;
                        report.exit_reason = "error".to_string();
                        write_loop_status(
                            &status_path,
                            LoopStatusPayload {
                                state: "error".to_string(),
                                iteration: state.iteration,
                                updated_at: now_rfc3339()?,
                                exit_reason: Some(report.exit_reason.clone()),
                                last_summary: Some(err.message.clone()),
                                last_error: Some(err.message.clone()),
                            },
                        )?;
                        if let Some(progress) = &opts.progress {
                            progress(LoopEvent {
                                iteration: state.iteration,
                                status: LOOP_EVENT_ERROR.to_string(),
                                details: err.message.clone(),
                            });
                        }
                        break;
                    }
                };

                update_test_result(&mut work_status, &item_key, test_outcome.clone(), &now_rfc3339()?);

                let pbt = work_status
                    .items
                    .get(&item_key)
                    .map(|item| item.pbt)
                    .unwrap_or(false);
                if pbt && test_outcome.passed {
                    mark_item_failed(
                        &mut work_status,
                        &item_key,
                        WorkItemState::Blocked,
                        "PBT tests passed on first run; must fail first",
                        &now_rfc3339()?,
                    );
                    write_work_status(&work_status_path, &work_status)?;
                    report.exit_reason = "pbt_passed_first".to_string();
                    write_loop_status(
                        &status_path,
                        LoopStatusPayload {
                            state: "blocked".to_string(),
                            iteration: state.iteration,
                            updated_at: now_rfc3339()?,
                            exit_reason: Some(report.exit_reason.clone()),
                            last_summary: Some("PBT tests passed on first run".to_string()),
                            last_error: None,
                        },
                    )?;
                    if let Some(progress) = &opts.progress {
                        progress(LoopEvent {
                            iteration: state.iteration,
                            status: LOOP_EVENT_EXIT.to_string(),
                            details: "pbt_passed_first".to_string(),
                        });
                    }
                    break;
                }

                mark_test_complete(
                    &mut work_status,
                    &item_key,
                    &status.summary,
                    &now_rfc3339()?,
                    !pbt || !test_outcome.passed,
                );
                write_work_status(&work_status_path, &work_status)?;

                write_loop_status(
                    &status_path,
                    LoopStatusPayload {
                        state: "running".to_string(),
                        iteration: state.iteration,
                        updated_at: now_rfc3339()?,
                        exit_reason: None,
                        last_summary: Some(format!(
                            "tests written for {} (task {})",
                            test.key, task.key
                        )),
                        last_error: if test_outcome.passed {
                            None
                        } else {
                            Some(format!(
                                "tests failing as expected (exit {})",
                                test_outcome.exit_code
                            ))
                        },
                    },
                )?;
            }
            WorkItemSelection::Task { task, tests } => {
                let test_outcome = match run_test_command(&root, &cfg.test_command, "validate") {
                    Ok(value) => value,
                    Err(err) => {
                        mark_item_failed(
                            &mut work_status,
                            &item_key,
                            WorkItemState::Error,
                            &err.message,
                            &now_rfc3339()?,
                        );
                        write_work_status(&work_status_path, &work_status)?;
                        report.exit_reason = "error".to_string();
                        write_loop_status(
                            &status_path,
                            LoopStatusPayload {
                                state: "error".to_string(),
                                iteration: state.iteration,
                                updated_at: now_rfc3339()?,
                                exit_reason: Some(report.exit_reason.clone()),
                                last_summary: Some(err.message.clone()),
                                last_error: Some(err.message.clone()),
                            },
                        )?;
                        if let Some(progress) = &opts.progress {
                            progress(LoopEvent {
                                iteration: state.iteration,
                                status: LOOP_EVENT_ERROR.to_string(),
                                details: err.message.clone(),
                            });
                        }
                        break;
                    }
                };

                update_test_result(&mut work_status, &item_key, test_outcome.clone(), &now_rfc3339()?);

                if !test_outcome.passed {
                    mark_item_failed(
                        &mut work_status,
                        &item_key,
                        WorkItemState::Error,
                        "tests failed after implementation",
                        &now_rfc3339()?,
                    );
                    write_work_status(&work_status_path, &work_status)?;
                    report.exit_reason = "tests_failed".to_string();
                    write_loop_status(
                        &status_path,
                        LoopStatusPayload {
                            state: "error".to_string(),
                            iteration: state.iteration,
                            updated_at: now_rfc3339()?,
                            exit_reason: Some(report.exit_reason.clone()),
                            last_summary: Some("tests failed after implementation".to_string()),
                            last_error: Some(format!(
                                "tests failed (exit {})",
                                test_outcome.exit_code
                            )),
                        },
                    )?;
                    if let Some(progress) = &opts.progress {
                        progress(LoopEvent {
                            iteration: state.iteration,
                            status: LOOP_EVENT_ERROR.to_string(),
                            details: "tests_failed".to_string(),
                        });
                    }
                    break;
                }

                mark_task_complete(&mut work_status, &task.key, &status.summary, &now_rfc3339()?);
                for test in &tests {
                    mark_test_validated(&mut work_status, &test.key, &now_rfc3339()?);
                }
                write_work_status(&work_status_path, &work_status)?;

                write_loop_status(
                    &status_path,
                    LoopStatusPayload {
                        state: "running".to_string(),
                        iteration: state.iteration,
                        updated_at: now_rfc3339()?,
                        exit_reason: None,
                        last_summary: Some(format!("implemented {}", task.key)),
                        last_error: None,
                    },
                )?;
            }
        }

        if let Some(progress) = &opts.progress {
            progress(LoopEvent {
                iteration: state.iteration,
                status: LOOP_EVENT_DONE.to_string(),
                details: String::new(),
            });
        }
    }

    report.iterations = state.iteration;
    Ok(report)
}

#[derive(Clone)]
enum WorkItemSelection {
    Test { test: TestSpec, task: TaskSpec, pbt: bool },
    Task { task: TaskSpec, tests: Vec<TestSpec> },
}

impl WorkItemSelection {
    fn key(&self) -> &str {
        match self {
            WorkItemSelection::Test { test, .. } => &test.key,
            WorkItemSelection::Task { task, .. } => &task.key,
        }
    }

    fn item_type(&self) -> &str {
        match self {
            WorkItemSelection::Test { .. } => "test",
            WorkItemSelection::Task { .. } => "task",
        }
    }

    fn phase(&self) -> &str {
        match self {
            WorkItemSelection::Test { .. } => "tests",
            WorkItemSelection::Task { .. } => "implement",
        }
    }

    fn is_pbt(&self) -> bool {
        match self {
            WorkItemSelection::Test { pbt, .. } => *pbt,
            WorkItemSelection::Task { .. } => false,
        }
    }

    fn describe(&self) -> String {
        match self {
            WorkItemSelection::Test { test, task, .. } => {
                format!("test {} (task {})", test.key, task.key)
            }
            WorkItemSelection::Task { task, .. } => format!("task {}", task.key),
        }
    }

    fn prompt_inputs(&self, step: &RunStep) -> Vec<String> {
        let mut inputs = Vec::new();
        inputs.push("loopr/state/handoff.md".to_string());
        inputs.push("specs/task-order.yaml".to_string());
        inputs.push("specs/test-order.yaml".to_string());
        match self {
            WorkItemSelection::Test { test, task, .. } => {
                inputs.push(task.file.clone());
                inputs.push(test.file.clone());
            }
            WorkItemSelection::Task { task, tests } => {
                inputs.push(task.file.clone());
                for test in tests {
                    inputs.push(test.file.clone());
                }
            }
        }
        for output in &step.outputs {
            if !inputs.iter().any(|value| value == output) {
                inputs.push(output.clone());
            }
        }
        dedupe_inputs(inputs)
    }
}

fn dedupe_inputs(inputs: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for input in inputs {
        if seen.insert(input.clone()) {
            out.push(input);
        }
    }
    out
}

fn select_next_item(
    tasks: &[TaskSpec],
    tests: &[TestSpec],
    task_by_id: &HashMap<i64, TaskSpec>,
    test_by_id: &HashMap<i64, TestSpec>,
    tests_by_task: &HashMap<i64, Vec<TestSpec>>,
    status: &WorkStatusFile,
) -> Option<WorkItemSelection> {
    for test in tests {
        if is_test_runnable(test, task_by_id, test_by_id, status) {
            let task = task_by_id.get(&test.task_id)?.clone();
            let pbt = status
                .items
                .get(&test.key)
                .map(|item| item.pbt)
                .unwrap_or(false);
            return Some(WorkItemSelection::Test {
                test: test.clone(),
                task,
                pbt,
            });
        }
    }
    for task in tasks {
        if is_task_runnable(task, task_by_id, tests_by_task, test_by_id, status) {
            let tests = tests_by_task
                .get(&task.id)
                .cloned()
                .unwrap_or_default();
            return Some(WorkItemSelection::Task {
                task: task.clone(),
                tests,
            });
        }
    }
    None
}

fn is_task_runnable(
    task: &TaskSpec,
    task_by_id: &HashMap<i64, TaskSpec>,
    tests_by_task: &HashMap<i64, Vec<TestSpec>>,
    test_by_id: &HashMap<i64, TestSpec>,
    status: &WorkStatusFile,
) -> bool {
    let item = match status.items.get(&task.key) {
        Some(value) => value,
        None => return false,
    };
    if item.state != WorkItemState::NotStarted {
        return false;
    }
    if !task_deps_complete(&task.depends_on, task_by_id, status) {
        return false;
    }
    let tests = match tests_by_task.get(&task.id) {
        Some(value) => value,
        None => return true,
    };
    for test in tests {
        let test_item = match status.items.get(&test.key) {
            Some(value) => value,
            None => return false,
        };
        if test_item.state == WorkItemState::Blocked || test_item.state == WorkItemState::Error {
            return false;
        }
        if !test_item.tests_written {
            return false;
        }
        if !test_deps_complete(&test.depends_on, test_by_id, status) {
            return false;
        }
    }
    true
}

fn is_test_runnable(
    test: &TestSpec,
    task_by_id: &HashMap<i64, TaskSpec>,
    test_by_id: &HashMap<i64, TestSpec>,
    status: &WorkStatusFile,
) -> bool {
    let item = match status.items.get(&test.key) {
        Some(value) => value,
        None => return false,
    };
    if item.state != WorkItemState::NotStarted {
        return false;
    }
    let task = match task_by_id.get(&test.task_id) {
        Some(value) => value,
        None => return false,
    };
    if !task_deps_complete(&task.depends_on, task_by_id, status) {
        return false;
    }
    if !test_deps_complete(&test.depends_on, test_by_id, status) {
        return false;
    }
    true
}

fn task_deps_complete(
    dep_ids: &[i64],
    task_by_id: &HashMap<i64, TaskSpec>,
    status: &WorkStatusFile,
) -> bool {
    for dep_id in dep_ids {
        let task = match task_by_id.get(dep_id) {
            Some(value) => value,
            None => return false,
        };
        let item = match status.items.get(&task.key) {
            Some(value) => value,
            None => return false,
        };
        if item.state != WorkItemState::Complete {
            return false;
        }
    }
    true
}

fn test_deps_complete(
    dep_ids: &[i64],
    test_by_id: &HashMap<i64, TestSpec>,
    status: &WorkStatusFile,
) -> bool {
    for dep_id in dep_ids {
        let dep = match test_by_id.get(dep_id) {
            Some(value) => value,
            None => return false,
        };
        let item = match status.items.get(&dep.key) {
            Some(value) => value,
            None => return false,
        };
        if !item.tests_written {
            return false;
        }
        if item.state != WorkItemState::Complete {
            return false;
        }
    }
    true
}

fn blocked_reason(status: &WorkStatusFile) -> Option<String> {
    for item in status.items.values() {
        if item.state == WorkItemState::Blocked {
            return Some("blocked".to_string());
        }
        if item.state == WorkItemState::Error {
            return Some("error".to_string());
        }
    }
    None
}

fn all_tasks_complete(tasks: &[TaskSpec], status: &WorkStatusFile) -> bool {
    tasks.iter().all(|task| {
        status
            .items
            .get(&task.key)
            .map(|item| item.state == WorkItemState::Complete)
            .unwrap_or(false)
    })
}

fn all_tests_validated(tests: &[TestSpec], status: &WorkStatusFile) -> bool {
    tests.iter().all(|test| {
        status
            .items
            .get(&test.key)
            .map(|item| item.tests_validated)
            .unwrap_or(false)
    })
}

fn update_item_in_progress(status: &mut WorkStatusFile, key: &str, phase: &str, now: &str) {
    if let Some(item) = status.items.get_mut(key) {
        item.state = WorkItemState::InProgress;
        item.attempts = item.attempts.saturating_add(1);
        item.last_updated = now.to_string();
        item.last_summary = Some(format!("phase {}", phase));
        item.last_error = None;
    }
    status.updated_at = now.to_string();
}

fn mark_item_failed(
    status: &mut WorkStatusFile,
    key: &str,
    state: WorkItemState,
    message: &str,
    now: &str,
) {
    if let Some(item) = status.items.get_mut(key) {
        item.state = state;
        item.last_updated = now.to_string();
        item.last_error = Some(message.to_string());
    }
    status.updated_at = now.to_string();
}

fn mark_test_complete(
    status: &mut WorkStatusFile,
    key: &str,
    summary: &str,
    now: &str,
    tests_written: bool,
) {
    if let Some(item) = status.items.get_mut(key) {
        item.state = WorkItemState::Complete;
        item.tests_written = tests_written;
        item.last_updated = now.to_string();
        if !summary.is_empty() {
            item.last_summary = Some(summary.to_string());
        }
        if item.last_error.is_some() && tests_written {
            item.last_error = None;
        }
    }
    status.updated_at = now.to_string();
}

fn mark_test_validated(status: &mut WorkStatusFile, key: &str, now: &str) {
    if let Some(item) = status.items.get_mut(key) {
        item.tests_validated = true;
        item.last_updated = now.to_string();
    }
    status.updated_at = now.to_string();
}

fn mark_task_complete(status: &mut WorkStatusFile, key: &str, summary: &str, now: &str) {
    if let Some(item) = status.items.get_mut(key) {
        item.state = WorkItemState::Complete;
        item.last_updated = now.to_string();
        if !summary.is_empty() {
            item.last_summary = Some(summary.to_string());
        }
    }
    status.updated_at = now.to_string();
}

fn update_test_result(
    status: &mut WorkStatusFile,
    key: &str,
    result: TestRunResult,
    now: &str,
) {
    if let Some(item) = status.items.get_mut(key) {
        item.last_test = Some(result);
        item.last_updated = now.to_string();
    }
    status.updated_at = now.to_string();
}

fn run_codex_for_prompt(
    root: &Path,
    cfg: &LoopConfig,
    codex_args: &[String],
    prompt: String,
) -> LooprResult<CodexRun> {
    let mut args = vec!["--cd".to_string(), root.display().to_string()];
    args.extend(codex_args.iter().cloned());
    args.push(prompt);

    if cfg.codex_timeout_minutes > 0 {
        run_codex_with_timeout(
            &args,
            &CodexOptions {
                loopr_root: Some(root.to_path_buf()),
                mode: CodexMode::Exec,
            },
            Duration::from_secs((cfg.codex_timeout_minutes as u64) * 60),
        )
    } else {
        run_codex(
            &args,
            &CodexOptions {
                loopr_root: Some(root.to_path_buf()),
                mode: CodexMode::Exec,
            },
        )
    }
}

fn run_test_command(root: &Path, command: &str, phase: &str) -> LooprResult<TestRunResult> {
    let command = command.trim();
    if command.is_empty() {
        return Err(LooprError::new("TEST_COMMAND is empty"));
    }
    let output = Command::new("sh")
        .arg("-lc")
        .arg(command)
        .current_dir(root)
        .output()
        .map_err(|err| LooprError::new(format!("run test command: {}", err)))?;
    let exit_code = output.status.code().unwrap_or(1);
    let passed = exit_code == 0;
    let _ = output;
    Ok(TestRunResult {
        exit_code,
        passed,
        ran_at: now_rfc3339()?,
        phase: phase.to_string(),
    })
}

fn detect_pbt(test: &TestSpec, root: &Path) -> LooprResult<bool> {
    if let Some(kind) = &test.kind {
        if kind.eq_ignore_ascii_case("pbt") {
            return Ok(true);
        }
    }
    let path = root.join(&test.file);
    let data = std::fs::read_to_string(&path).map_err(|err| {
        LooprError::new(format!("read {}: {}", path.display(), err))
    })?;
    let lower = data.to_lowercase();
    let keywords = [
        "property-based",
        "property based",
        "pbt",
        "proptest",
        "quickcheck",
        "fast-check",
        "fastcheck",
    ];
    Ok(keywords.iter().any(|needle| lower.contains(needle)))
}

fn build_per_task_prompt(
    step: &RunStep,
    handoff_path: &Path,
    iteration: i64,
    item_key: &str,
    item_type: &str,
    phase: &str,
    inputs: &[String],
    pbt: bool,
) -> String {
    let mut lines = vec![
        format!("Loopr loop iteration: {}", iteration),
        format!("Item: {} ({})", item_key, item_type),
        format!("Phase: {}", phase),
        String::new(),
        format!("Prompt: {}", step.skill),
        String::new(),
        "Allowed inputs:".to_string(),
    ];
    for input in inputs {
        lines.push(format!("- {}", input));
    }
    if step.allow_repo_read {
        lines.push("- Repo files as needed (read-only).".to_string());
    }
    lines.push(String::new());
    lines.push("Required outputs:".to_string());
    for output in &step.outputs {
        lines.push(format!("- {}", output));
    }

    lines.push(String::new());
    lines.push("Rules:".to_string());
    if step.allow_repo_read {
        lines.push(
            "- Read the allowed inputs and any repo files needed for implementation.".to_string(),
        );
        lines.push("- Avoid broad scans; open only what you need.".to_string());
    } else {
        lines.push("- Read only the allowed inputs.".to_string());
        lines.push("- Do not scan the repo.".to_string());
    }
    lines.push(
        "- If required inputs are missing, stop and ask to run the appropriate step.".to_string(),
    );
    lines.push(format!(
        "- Append a completion note to {} (decisions, open questions, tests).",
        handoff_path.display()
    ));
    match phase {
        "tests" => {
            lines.push("- Write tests only; do not implement production code beyond minimal scaffolding.".to_string());
            lines.push("- Ensure tests can run (Loopr will execute the test command after this session).".to_string());
            if pbt {
                lines.push("- This is a PBT test: tests must fail on the first run.".to_string());
            }
        }
        "implement" => {
            lines.push("- Implement the task to satisfy the tests.".to_string());
            lines.push("- Ensure tests pass (Loopr will execute the test command after this session).".to_string());
        }
        _ => {}
    }
    lines.push(
        "- Only set EXIT_SIGNAL: true when all tasks are complete and tests are green.".to_string(),
    );
    lines.push("- Always include the status block at the end of your response.".to_string());
    lines.push(String::new());
    lines.push("Status block format (required):".to_string());
    lines.push(LOOPR_STATUS_START.to_string());
    lines.push("STATUS: IN_PROGRESS | COMPLETE | BLOCKED | ERROR".to_string());
    lines.push("EXIT_SIGNAL: true | false".to_string());
    lines.push(format!("ITEM_KEY: {}", item_key));
    lines.push(format!("ITEM_TYPE: {}", item_type));
    lines.push(format!("PHASE: {}", phase));
    lines.push("SUMMARY: <short summary>".to_string());
    lines.push(LOOPR_STATUS_END.to_string());
    lines.push(String::new());
    lines.push(format!("Run the prompt: {}", step.skill));
    lines.join("\n")
}

fn build_loop_prompt(step: &RunStep, handoff_path: &Path, iteration: i64) -> String {
    let mut lines = vec![format!("Loopr loop iteration: {}", iteration)];
    lines.extend(build_prompt_lines(step, "", handoff_path));
    lines.push(
        "- Only set EXIT_SIGNAL: true when all tasks are complete and tests are green.".to_string(),
    );
    lines.push("- Always include the status block at the end of your response.".to_string());
    lines.push(String::new());
    lines.push("Status block format (required):".to_string());
    lines.push(LOOPR_STATUS_START.to_string());
    lines.push("STATUS: IN_PROGRESS | COMPLETE | BLOCKED | ERROR".to_string());
    lines.push("EXIT_SIGNAL: true | false".to_string());
    lines.push("SUMMARY: <short summary>".to_string());
    lines.push(LOOPR_STATUS_END.to_string());
    lines.push(String::new());
    lines.push(format!("Run the prompt: {}", step.skill));
    lines.join("\n")
}

fn evaluate_loop_exit(
    cfg: &LoopConfig,
    status: &LooprStatus,
    status_found: bool,
    state: &mut LoopState,
) -> (String, String) {
    if status_found {
        state.missing_status_count = 0;
    } else {
        state.missing_status_count += 1;
    }
    if !status_found && state.missing_status_count >= cfg.max_missing_status {
        return ("missing_status".to_string(), "error".to_string());
    }

    if status.exit_signal || status.status == "COMPLETE" {
        return ("completed".to_string(), "complete".to_string());
    }
    match status.status.as_str() {
        "BLOCKED" => ("blocked".to_string(), "blocked".to_string()),
        "ERROR" => ("error".to_string(), "error".to_string()),
        _ => (String::new(), "running".to_string()),
    }
}

fn write_loop_status(path: &Path, status: LoopStatusPayload) -> LooprResult<()> {
    let data = serde_json::to_vec_pretty(&status)
        .map_err(|err| LooprError::new(format!("serialize status: {}", err)))?;
    let mut data = data;
    data.push(b'\n');
    write_file_atomic(path, &data, 0o644)
}

fn ensure_handoff(root: &Path) -> LooprResult<PathBuf> {
    let path = root.join("loopr").join("state").join("handoff.md");
    if path.exists() {
        return Ok(path);
    }
    if let Some(parent) = path.parent() {
        ensure_dir(parent, 0o755)?;
    }
    let header = format!("# Loopr Handoff\n\nInitialized: {}\n\n", now_rfc3339()?);
    write_file_atomic(&path, header.as_bytes(), 0o644)?;
    Ok(path)
}

fn now_rfc3339() -> LooprResult<String> {
    OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .map_err(|err| LooprError::new(format!("format time: {}", err)))
}

fn codex_error(run: &CodexRun) -> Option<LooprError> {
    if run.timed_out {
        return Some(LooprError::new("codex timed out"));
    }
    if let Some(message) = &run.error_message {
        return Some(LooprError::new(message.clone()));
    }
    if run.exit_code != 0 {
        return Some(LooprError::new(format!("exit status {}", run.exit_code)));
    }
    None
}

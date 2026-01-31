use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::Serialize;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::ops::codex::{CodexOptions, CodexRun, CodexSession, run_codex, run_codex_with_timeout};
use crate::ops::fs::{ensure_dir, write_file_atomic};
use crate::ops::loop_config::{LoopConfig, load_loop_config};
use crate::ops::loop_status::{
    LOOPR_STATUS_END, LOOPR_STATUS_START, LooprStatus, parse_loopr_status_from_log,
};
use crate::ops::loopr_root::resolve_loopr_root;
use crate::ops::run::{RunStep, build_prompt_lines, default_run_steps, find_step};
use crate::{LooprError, LooprResult};

pub struct LoopOptions {
    pub loopr_root: Option<PathBuf>,
    pub max_iterations: i64,
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
                },
                Duration::from_secs((cfg.codex_timeout_minutes as u64) * 60),
            )?
        } else {
            run_codex(
                &args,
                &CodexOptions {
                    loopr_root: Some(root.clone()),
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

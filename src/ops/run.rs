use std::collections::HashSet;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::ops::codex::{CodexMode, CodexOptions, CodexRun, CodexSession, run_codex};
use crate::ops::fs::write_file_atomic;
use crate::ops::loopr_root::resolve_loopr_root;
use crate::{LooprError, LooprResult};

#[derive(Debug, Clone)]
pub struct RunStep {
    pub name: String,
    pub skill: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub requires_seed: bool,
    pub always_run: bool,
    pub allow_repo_read: bool,
}

pub struct RunOptions {
    pub loopr_root: Option<PathBuf>,
    pub from: String,
    pub to: String,
    pub step: String,
    pub seed: String,
    pub confirm: bool,
    pub no_prompt: bool,
    pub codex: bool,
    pub codex_args: Vec<String>,
    pub progress: Option<Box<dyn Fn(ProgressEvent)>>,
}

pub struct RunReport {
    pub steps: Vec<RunStep>,
    pub executed: Vec<RunStep>,
    pub skipped: Vec<RunStep>,
    pub last_session: Option<CodexSession>,
}

pub struct ProgressEvent {
    pub step: RunStep,
    pub index: usize,
    pub total: usize,
    pub status: String,
}

pub const PROGRESS_START: &str = "start";
pub const PROGRESS_DONE: &str = "done";
pub const PROGRESS_ERROR: &str = "error";

pub fn run_workflow(opts: RunOptions) -> LooprResult<RunReport> {
    let cwd = std::env::current_dir()?;
    let root = if opts.codex {
        let (root, _) = resolve_loopr_root(&cwd, opts.loopr_root.as_deref())?;
        root
    } else {
        resolve_plan_root(&cwd, opts.loopr_root.as_deref())?
    };

    let append_prompt = !opts.no_prompt;

    if opts.codex && !append_prompt {
        let mut args = vec!["--cd".to_string(), root.display().to_string()];
        args.extend(opts.codex_args.clone());
        let run = run_codex(
            &args,
            &CodexOptions {
                loopr_root: Some(root.clone()),
                mode: CodexMode::Interactive,
            },
        )?;
        let err = codex_error(&run);
        let report = RunReport {
            steps: Vec::new(),
            executed: Vec::new(),
            skipped: Vec::new(),
            last_session: Some(run.session),
        };
        if let Some(err) = err {
            return Err(err);
        }
        return Ok(report);
    }

    let handoff_path = if opts.codex {
        Some(ensure_handoff(&root)?)
    } else {
        None
    };

    let steps = plan_steps(&opts)?;
    let mut report = RunReport {
        steps: steps.clone(),
        executed: Vec::new(),
        skipped: Vec::new(),
        last_session: None,
    };

    if !opts.codex {
        return Ok(report);
    }

    let total = steps.len();
    for step in steps {
        let idx = report.executed.len() + report.skipped.len() + 1;
        if append_prompt && step.requires_seed && opts.seed.trim().is_empty() {
            return Err(LooprError::new(format!(
                "seed prompt required for {} (use --seed-prompt)",
                step.name
            )));
        }
        if opts.confirm {
            let ok = confirm_step(&step.name)?;
            if !ok {
                return Err(LooprError::new("run cancelled"));
            }
        }
        if let Some(progress) = &opts.progress {
            progress(ProgressEvent {
                step: step.clone(),
                index: idx,
                total,
                status: PROGRESS_START.to_string(),
            });
        }

        let mut args = vec!["--cd".to_string(), root.display().to_string()];
        args.extend(opts.codex_args.clone());
        if append_prompt {
            let prompt = build_prompt(&step, &opts.seed, handoff_path.as_ref().unwrap());
            args.push(prompt);
        }

        let run = run_codex(
            &args,
            &CodexOptions {
                loopr_root: Some(root.clone()),
                mode: CodexMode::Exec,
            },
        )?;
        let err = codex_error(&run);
        report.last_session = Some(run.session);
        if let Some(err) = err {
            if let Some(progress) = &opts.progress {
                progress(ProgressEvent {
                    step: step.clone(),
                    index: idx,
                    total,
                    status: PROGRESS_ERROR.to_string(),
                });
            }
            return Err(err);
        }
        if let Some(progress) = &opts.progress {
            progress(ProgressEvent {
                step: step.clone(),
                index: idx,
                total,
                status: PROGRESS_DONE.to_string(),
            });
        }
        report.executed.push(step);
    }

    Ok(report)
}

fn resolve_plan_root(cwd: &Path, override_root: Option<&Path>) -> LooprResult<PathBuf> {
    if let Some(root) = override_root {
        let abs = if root.is_absolute() {
            root.to_path_buf()
        } else {
            cwd.join(root)
        };
        return Ok(abs);
    }
    Ok(cwd.to_path_buf())
}

pub fn plan_steps(opts: &RunOptions) -> LooprResult<Vec<RunStep>> {
    let steps = default_run_steps();
    if !opts.step.is_empty() {
        let step = find_step(&steps, &opts.step)
            .ok_or_else(|| LooprError::new(format!("unknown step: {}", opts.step)))?;
        return Ok(vec![step]);
    }
    if !opts.from.is_empty() || !opts.to.is_empty() {
        return select_range(&steps, &opts.from, &opts.to);
    }
    Ok(steps)
}

pub fn default_run_steps() -> Vec<RunStep> {
    vec![
        RunStep {
            name: "prd".to_string(),
            skill: "loopr-prd".to_string(),
            inputs: vec!["loopr/state/handoff.md".to_string()],
            outputs: vec!["specs/prd.md".to_string()],
            requires_seed: true,
            always_run: false,
            allow_repo_read: false,
        },
        RunStep {
            name: "spec".to_string(),
            skill: "loopr-specify".to_string(),
            inputs: vec![
                "loopr/state/handoff.md".to_string(),
                "specs/prd.md".to_string(),
            ],
            outputs: vec!["specs/spec.md".to_string()],
            requires_seed: false,
            always_run: false,
            allow_repo_read: false,
        },
        RunStep {
            name: "features".to_string(),
            skill: "loopr-features".to_string(),
            inputs: vec![
                "loopr/state/handoff.md".to_string(),
                "specs/spec.md".to_string(),
            ],
            outputs: vec![
                "specs/feature-order.yaml".to_string(),
                "specs/feature-*.md".to_string(),
            ],
            requires_seed: false,
            always_run: false,
            allow_repo_read: false,
        },
        RunStep {
            name: "tasks".to_string(),
            skill: "loopr-tasks".to_string(),
            inputs: vec![
                "loopr/state/handoff.md".to_string(),
                "specs/feature-order.yaml".to_string(),
                "specs/feature-*.md".to_string(),
            ],
            outputs: vec![
                "specs/task-order.yaml".to_string(),
                "specs/feature-*-task-*.md".to_string(),
            ],
            requires_seed: false,
            always_run: false,
            allow_repo_read: false,
        },
        RunStep {
            name: "tests".to_string(),
            skill: "loopr-tests".to_string(),
            inputs: vec![
                "loopr/state/handoff.md".to_string(),
                "specs/task-order.yaml".to_string(),
                "specs/feature-*-task-*.md".to_string(),
            ],
            outputs: vec![
                "specs/test-order.yaml".to_string(),
                "specs/feature-*-task-*-test-*.md".to_string(),
            ],
            requires_seed: false,
            always_run: false,
            allow_repo_read: false,
        },
        RunStep {
            name: "execute".to_string(),
            skill: "loopr-execute".to_string(),
            inputs: vec![
                "loopr/state/handoff.md".to_string(),
                "specs/task-order.yaml".to_string(),
                "specs/test-order.yaml".to_string(),
                "specs/feature-*-task-*.md".to_string(),
                "specs/feature-*-task-*-test-*.md".to_string(),
            ],
            outputs: vec!["specs/implementation-progress.md".to_string()],
            requires_seed: false,
            always_run: true,
            allow_repo_read: true,
        },
    ]
}

fn select_range(steps: &[RunStep], from: &str, to: &str) -> LooprResult<Vec<RunStep>> {
    let mut start = 0;
    let mut end = steps.len().saturating_sub(1);
    if !from.is_empty() {
        let idx = index_of_step(steps, from);
        if idx < 0 {
            return Err(LooprError::new(format!("unknown step: {}", from)));
        }
        start = idx as usize;
    }
    if !to.is_empty() {
        let idx = index_of_step(steps, to);
        if idx < 0 {
            return Err(LooprError::new(format!("unknown step: {}", to)));
        }
        end = idx as usize;
    }
    if start > end {
        return Err(LooprError::new(format!(
            "invalid step range: {} to {}",
            from, to
        )));
    }
    Ok(steps[start..=end].to_vec())
}

fn index_of_step(steps: &[RunStep], name: &str) -> i32 {
    for (idx, step) in steps.iter().enumerate() {
        if step.name == name {
            return idx as i32;
        }
    }
    -1
}

pub fn find_step(steps: &[RunStep], name: &str) -> Option<RunStep> {
    let idx = index_of_step(steps, name);
    if idx < 0 {
        return None;
    }
    steps.get(idx as usize).cloned()
}

pub fn build_prompt(step: &RunStep, seed: &str, handoff_path: &Path) -> String {
    let mut lines = Vec::new();
    lines.push(format!("Loopr step: {}", step.name));
    lines.extend(build_prompt_lines(step, seed, handoff_path));
    lines.push(String::new());
    lines.push(format!("Run the prompt: {}", step.skill));
    lines.join("\n")
}

pub fn build_prompt_lines(step: &RunStep, seed: &str, handoff_path: &Path) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!("Prompt: {}", step.skill));
    lines.push(String::new());
    lines.push("Allowed inputs:".to_string());

    let mut seen = HashSet::new();
    for input in &step.inputs {
        if seen.insert(input) {
            lines.push(format!("- {}", input));
        }
    }
    if step.allow_repo_read {
        lines.push("- Repo files as needed (read-only).".to_string());
    }
    lines.push(String::new());
    lines.push("Required outputs:".to_string());
    for output in &step.outputs {
        lines.push(format!("- {}", output));
    }

    if step.requires_seed {
        lines.push(String::new());
        lines.push("Seed prompt:".to_string());
        lines.push(seed.to_string());
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

    lines
}

fn ensure_handoff(root: &Path) -> LooprResult<PathBuf> {
    let path = root.join("loopr").join("state").join("handoff.md");
    if path.exists() {
        return Ok(path);
    }
    if let Some(parent) = path.parent() {
        crate::ops::fs::ensure_dir(parent, 0o755)?;
    }
    let ts = OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .map_err(|err| LooprError::new(format!("format time: {}", err)))?;
    let header = format!("# Loopr Handoff\n\nInitialized: {}\n\n", ts);
    write_file_atomic(&path, header.as_bytes(), 0o644)?;
    Ok(path)
}

fn confirm_step(name: &str) -> LooprResult<bool> {
    print!("Run step {}? [y/N]: ", name);
    io::stdout()
        .flush()
        .map_err(|err| LooprError::new(format!("flush stdout: {}", err)))?;
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|err| LooprError::new(format!("read stdin: {}", err)))?;
    let answer = input.trim().to_lowercase();
    Ok(answer == "y" || answer == "yes")
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

use clap::Parser;
use std::path::PathBuf;

use crate::ops;
use crate::version;
use crate::{LooprError, LooprResult};

#[derive(Parser, Debug)]
#[command(disable_help_flag = true)]
struct InitArgs {
    #[arg(long, default_value = ".")]
    root: String,
    #[arg(
        long = "no-agents",
        help = "Skip creating or injecting AGENTS.md during init."
    )]
    no_agents: bool,
}

#[derive(Parser, Debug)]
#[command(
    disable_help_flag = false,
    disable_version_flag = true,
    about = "Orchestrate the Loopr workflow steps (PRD -> Spec -> Features -> Tasks -> Tests -> Execute). Requires --codex or --dry-run. Use --from/--to to run a contiguous range, or --step for a single step. When --codex is set, the prompt and handoff rules are enforced; when --dry-run is set, no Codex session is started.",
    after_help = "Examples:\n  loopr run --codex --seed-prompt @seed.txt\n  loopr run --codex --from spec --to tests\n  loopr run --dry-run\n  loopr run --codex -- --model <model name>\n",
    help_template = "{about}\n\nUsage: {usage}\n\nOptions:\n{options}\n\n{after-help}"
)]
struct RunArgs {
    #[arg(
        long,
        help = "Start at this step (e.g., prd, spec, features, tasks, tests, execute).",
        long_help = "Start at this step (e.g., prd, spec, features, tasks, tests, execute). Runs through --to or the end if --to is omitted. Ignored if --step is set."
    )]
    from: Option<String>,
    #[arg(
        long,
        help = "End at this step (inclusive).",
        long_help = "End at this step (inclusive). Used with --from; ignored if --step is set."
    )]
    to: Option<String>,
    #[arg(
        long,
        help = "Run only this step (overrides --from/--to).",
        long_help = "Run only this step (overrides --from/--to). Useful for re-running a single stage."
    )]
    step: Option<String>,
    #[arg(
        long = "seed-prompt",
        help = "Seed prompt text or @path to read from a file.",
        long_help = "Seed prompt text or @path to read from a file. Required when running the prd step if specs/prd.md is missing."
    )]
    seed_prompt: Option<String>,
    #[arg(
        long,
        help = "Ask for confirmation before each step.",
        long_help = "Ask for confirmation before each step when running with --codex."
    )]
    confirm: bool,
    #[arg(
        long = "no-prompt",
        help = "Open Codex without a Loopr prompt (interactive mode).",
        long_help = "Open Codex without a Loopr prompt (interactive mode). Skips step planning and handoff enforcement."
    )]
    no_prompt: bool,
    #[arg(
        long,
        help = "Run with Codex (required unless --dry-run).",
        long_help = "Run with Codex (required unless --dry-run). All prompts are executed with Loopr's safety rules."
    )]
    codex: bool,
    #[arg(
        long = "dry-run",
        help = "Print planned steps without running Codex.",
        long_help = "Print planned steps without running Codex or reading prompts. Useful to preview which steps will run."
    )]
    dry_run: bool,
    #[arg(
        long = "loopr-root",
        help = "Override Loopr root (defaults to nearest loopr/repo-id).",
        long_help = "Override Loopr root (defaults to nearest loopr/repo-id). Use this when running from a different working directory."
    )]
    loopr_root: Option<String>,
}

#[derive(Parser, Debug)]
#[command(
    disable_help_flag = false,
    disable_version_flag = true,
    about = "Run repeated Loopr execute iterations with safety gates (exit signals, missing-status limits, and optional per-task mode). Default mode runs a single execute prompt per iteration. Use --per-task to run one Codex session per test/task item with tests-first enforcement, using specs/task-order.yaml and specs/test-order.yaml.",
    after_help = "Examples:\n  loopr loop\n  loopr loop --max-iterations 5\n  loopr loop --per-task\n  loopr loop --loopr-root /repo/app -- --model <model name>\n",
    help_template = "{about}\n\nUsage: {usage}\n\nOptions:\n{options}\n\n{after-help}"
)]
struct LoopArgs {
    #[arg(
        long = "loopr-root",
        help = "Override Loopr root (defaults to nearest loopr/repo-id).",
        long_help = "Override Loopr root (defaults to nearest loopr/repo-id). Use this when running from a different working directory."
    )]
    loopr_root: Option<String>,
    #[arg(
        long = "max-iterations",
        default_value_t = 0,
        help = "Stop after N iterations (0 = no limit).",
        long_help = "Stop after N iterations (0 = no limit). In --per-task mode, each item run counts as one iteration."
    )]
    max_iterations: i64,
    #[arg(
        long = "per-task",
        help = "Run one Codex session per test/task item.",
        long_help = "Run one Codex session per test/task item. Tests are written and executed first; implementation runs only after tests are written. PBT tests must fail on the first run. Progress tracked in loopr/state/work-status.json and tests run via TEST_COMMAND (default: `just test`)."
    )]
    per_task: bool,
}

#[derive(Parser, Debug)]
#[command(
    disable_help_flag = false,
    disable_version_flag = true,
    about = "Generate or refresh the Loopr docs index (loopr/state/docs-index.txt).",
    after_help = "Example:\n  loopr index\n  loopr index --loopr-root /repo/app\n",
    help_template = "{about}\n\nUsage: {usage}\n\nOptions:\n{options}\n\n{after-help}"
)]
struct IndexArgs {
    #[arg(
        long = "loopr-root",
        help = "Override Loopr root (defaults to nearest loopr/repo-id).",
        long_help = "Override Loopr root (defaults to nearest loopr/repo-id). Use this when running from a different working directory."
    )]
    loopr_root: Option<String>,
}

pub fn usage() {
    println!("loopr <command> [options]\n");
    println!("Commands:");
    println!("  init       Initialize Loopr metadata in a repo");
    println!("  run        Orchestrate Loopr steps (requires --codex or --dry-run)");
    println!("  loop       Run the Loopr execute loop with safety gates");
    println!("  index      Refresh the Loopr docs index (loopr/state/docs-index.txt)");
    println!("  version     Show version info");
}

pub fn run_init(args: Vec<String>) -> i32 {
    let mut argv = vec!["init".to_string()];
    argv.extend(args);
    let parsed = match InitArgs::try_parse_from(argv) {
        Ok(value) => value,
        Err(err) => return handle_clap_error(err),
    };

    let root = parsed.root.trim();
    let report = match ops::init::init(ops::init::InitOptions {
        root: if root.is_empty() {
            ".".into()
        } else {
            root.into()
        },
        rand: None,
        no_agents: parsed.no_agents,
    }) {
        Ok(report) => report,
        Err(err) => return fail(&err.to_string()),
    };

    println!("Repo root:   {}", report.root.display());
    println!("Repo ID:     {}", report.repo_id);
    println!("Transcripts: {}", report.transcripts_dir.display());
    0
}

pub fn run_run(args: Vec<String>) -> i32 {
    let (loopr_args, agent_args) = split_on_double_dash(&args);
    let (loopr_args, mut agent_args) = extract_codex_passthrough_flags(loopr_args, agent_args);

    let mut argv = vec!["run".to_string()];
    argv.extend(loopr_args.clone());
    let parsed = match RunArgs::try_parse_from(argv) {
        Ok(value) => value,
        Err(err) => return handle_clap_error(err),
    };

    if parsed.codex && parsed.dry_run {
        return fail("--codex and --dry-run are mutually exclusive");
    }

    let mut codex = parsed.codex;
    let mut seed_prompt = parsed.seed_prompt.unwrap_or_default();
    let mut confirm = parsed.confirm;
    let mut no_prompt = parsed.no_prompt;

    if parsed.dry_run {
        codex = false;
        agent_args.clear();
        seed_prompt = String::new();
        confirm = false;
        no_prompt = false;
    }

    if !agent_args.is_empty() && !codex && !parsed.dry_run {
        return fail("agent args provided but --codex not set");
    }
    if !codex && !parsed.dry_run {
        return fail("run requires --codex or --dry-run");
    }
    if codex && !no_prompt {
        match resolve_seed_prompt(seed_prompt) {
            Ok(value) => seed_prompt = value,
            Err(err) => return fail(&err.to_string()),
        }
    }

    let loopr_root = parsed
        .loopr_root
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from);
    let opts = ops::run::RunOptions {
        loopr_root,
        from: parsed.from.unwrap_or_default(),
        to: parsed.to.unwrap_or_default(),
        step: parsed.step.unwrap_or_default(),
        seed: seed_prompt,
        confirm,
        no_prompt,
        codex,
        codex_args: agent_args,
        progress: if codex {
            Some(Box::new(|event| {
                println!(
                    "Step {}/{} {}: {}",
                    event.index, event.total, event.step.name, event.status
                );
            }))
        } else {
            None
        },
    };

    let report = match ops::run::run_workflow(opts) {
        Ok(report) => report,
        Err(err) => return fail(&err.to_string()),
    };

    if !codex {
        for step in report.steps {
            println!("Step: {}", step.name);
            println!("  prompt: {}", step.skill);
            for input in step.inputs {
                println!("  input: {}", input);
            }
            for output in step.outputs {
                println!("  output: {}", output);
            }
        }
        return 0;
    }

    if let Some(session) = report.last_session {
        println!("Transcript: {}", session.log_path.display());
        println!("Metadata:   {}", session.meta_path.display());
    }
    0
}

pub fn run_loop(args: Vec<String>) -> i32 {
    let (loopr_args, agent_args) = split_on_double_dash(&args);

    let mut argv = vec!["loop".to_string()];
    argv.extend(loopr_args);
    let parsed = match LoopArgs::try_parse_from(argv) {
        Ok(value) => value,
        Err(err) => return handle_clap_error(err),
    };

    let loopr_root = parsed
        .loopr_root
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from);
    let opts = ops::loop_run::LoopOptions {
        loopr_root,
        max_iterations: parsed.max_iterations,
        per_task: parsed.per_task,
        codex_args: agent_args,
        progress: Some(Box::new(|event| {
            if !event.details.is_empty() {
                println!(
                    "Loop {} {}: {}",
                    event.iteration, event.status, event.details
                );
            } else {
                println!("Loop {} {}", event.iteration, event.status);
            }
        })),
    };

    let report = match ops::loop_run::run_loop(opts) {
        Ok(report) => report,
        Err(err) => return fail(&err.to_string()),
    };

    if !report.exit_reason.is_empty() {
        println!("Exit reason: {}", report.exit_reason);
    }
    if let Some(session) = report.last_session {
        println!("Transcript: {}", session.log_path.display());
        println!("Metadata:   {}", session.meta_path.display());
    }
    0
}

pub fn run_index(args: Vec<String>) -> i32 {
    let mut argv = vec!["index".to_string()];
    argv.extend(args);
    let parsed = match IndexArgs::try_parse_from(argv) {
        Ok(value) => value,
        Err(err) => return handle_clap_error(err),
    };

    let loopr_root = parsed
        .loopr_root
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from);

    let cwd = match std::env::current_dir() {
        Ok(value) => value,
        Err(err) => return fail(&format!("current dir: {}", err)),
    };
    let (root, _) = match ops::loopr_root::resolve_loopr_root(&cwd, loopr_root.as_deref()) {
        Ok(value) => value,
        Err(err) => return fail(&err.to_string()),
    };

    let index_path = match ops::docs_index::write_docs_index(&root) {
        Ok(value) => value,
        Err(err) => return fail(&err.to_string()),
    };
    println!("Docs index: {}", index_path.display());
    0
}

pub fn run_version() -> i32 {
    println!("loopr {}", version::VERSION);
    if !version::COMMIT.is_empty() {
        println!("commit: {}", version::COMMIT);
    }
    if !version::DATE.is_empty() {
        println!("date: {}", version::DATE);
    }
    0
}

fn handle_clap_error(err: clap::Error) -> i32 {
    use clap::error::ErrorKind;
    match err.kind() {
        ErrorKind::DisplayHelp | ErrorKind::DisplayVersion => {
            print!("{}", err);
            0
        }
        _ => {
            eprint!("{}", err);
            2
        }
    }
}

fn fail(message: &str) -> i32 {
    eprintln!("error: {}", message);
    1
}

fn resolve_seed_prompt(raw: String) -> LooprResult<String> {
    let value = raw.trim();
    if !value.starts_with('@') {
        return Ok(raw);
    }
    let path = value.trim_start_matches('@').trim();
    if path.is_empty() {
        return Err(LooprError::new("seed prompt file path is empty"));
    }
    let content = std::fs::read_to_string(path)
        .map_err(|err| LooprError::new(format!("read seed prompt file {}: {}", path, err)))?;
    Ok(content)
}

pub fn split_on_double_dash(args: &[String]) -> (Vec<String>, Vec<String>) {
    for (idx, arg) in args.iter().enumerate() {
        if arg == "--" {
            if idx == 0 {
                return (Vec::new(), args[idx + 1..].to_vec());
            }
            return (args[..idx].to_vec(), args[idx + 1..].to_vec());
        }
    }
    (args.to_vec(), Vec::new())
}

pub fn extract_codex_passthrough_flags(
    loopr_args: Vec<String>,
    agent_args: Vec<String>,
) -> (Vec<String>, Vec<String>) {
    if !agent_args.is_empty() || !has_codex_flag(&loopr_args) {
        return (loopr_args, agent_args);
    }

    let mut filtered = Vec::with_capacity(loopr_args.len());
    let mut codex_args = agent_args;
    for arg in loopr_args {
        if is_codex_help_flag(&arg) {
            codex_args.push(arg);
            continue;
        }
        filtered.push(arg);
    }
    (filtered, codex_args)
}

pub fn has_codex_flag(args: &[String]) -> bool {
    args.iter()
        .any(|arg| arg == "--codex" || arg.starts_with("--codex="))
}

pub fn is_codex_help_flag(arg: &str) -> bool {
    matches!(arg, "-h" | "-help" | "--help" | "-V" | "--version")
        || arg.starts_with("--help=")
        || arg.starts_with("--version=")
}

#[cfg(test)]
mod tests {
    use super::resolve_seed_prompt;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_seed_path(name: &str) -> std::path::PathBuf {
        let mut dir = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        dir.push(format!("loopr-seed-{}-{}.txt", name, nanos));
        dir
    }

    #[test]
    fn resolve_seed_prompt_literal() {
        let value = resolve_seed_prompt("hello world".to_string()).unwrap();
        assert_eq!(value, "hello world");
    }

    #[test]
    fn resolve_seed_prompt_file() {
        let path = temp_seed_path("file");
        std::fs::write(&path, "seed from file\n").unwrap();
        let value = resolve_seed_prompt(format!("@{}", path.display())).unwrap();
        assert_eq!(value, "seed from file\n");
    }

    #[test]
    fn resolve_seed_prompt_empty_path() {
        let err = resolve_seed_prompt("@".to_string()).unwrap_err();
        assert!(err.to_string().contains("seed prompt file path is empty"));
    }
}

use clap::Parser;
use std::path::PathBuf;

use crate::ops;
use crate::version;

#[derive(Parser, Debug)]
#[command(disable_help_flag = true)]
struct InitArgs {
    #[arg(long, default_value = ".")]
    root: String,
}

#[derive(Parser, Debug)]
#[command(disable_help_flag = false, disable_version_flag = true)]
struct RunArgs {
    #[arg(long)]
    from: Option<String>,
    #[arg(long)]
    to: Option<String>,
    #[arg(long)]
    step: Option<String>,
    #[arg(long = "seed-prompt")]
    seed_prompt: Option<String>,
    #[arg(long)]
    confirm: bool,
    #[arg(long = "no-prompt")]
    no_prompt: bool,
    #[arg(long)]
    codex: bool,
    #[arg(long = "dry-run")]
    dry_run: bool,
    #[arg(long = "loopr-root")]
    loopr_root: Option<String>,
}

#[derive(Parser, Debug)]
#[command(disable_help_flag = false, disable_version_flag = true)]
struct LoopArgs {
    #[arg(long = "loopr-root")]
    loopr_root: Option<String>,
    #[arg(long = "max-iterations", default_value_t = 0)]
    max_iterations: i64,
}

pub fn usage() {
    println!("loopr <command> [options]\n");
    println!("Commands:");
    println!("  init       Initialize Loopr metadata in a repo");
    println!("  run        Orchestrate Loopr steps (requires --codex or --dry-run)");
    println!("  loop       Run the Loopr execute loop with safety gates");
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

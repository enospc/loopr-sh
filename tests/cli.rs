use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use loopr::cli::{extract_codex_passthrough_flags, split_on_double_dash};

fn temp_dir(name: &str) -> std::path::PathBuf {
    let mut dir = std::env::temp_dir();
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    dir.push(format!("loopr-test-{}-{}", name, nanos));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn run_loopr(args: &[&str]) -> (String, i32) {
    let output = Command::new(env!("CARGO_BIN_EXE_loopr"))
        .args(args)
        .output()
        .expect("run loopr");
    let code = output.status.code().unwrap_or(1);
    let mut text = String::new();
    text.push_str(&String::from_utf8_lossy(&output.stdout));
    text.push_str(&String::from_utf8_lossy(&output.stderr));
    (text, code)
}

#[test]
fn test_split_on_double_dash() {
    let args = vec!["--".to_string(), "--help".to_string()];
    let (loopr_args, codex_args) = split_on_double_dash(&args);
    assert!(loopr_args.is_empty());
    assert_eq!(codex_args, vec!["--help".to_string()]);

    let args = vec![
        "--loopr-root".to_string(),
        "/tmp/root".to_string(),
        "--".to_string(),
        "--help".to_string(),
    ];
    let (loopr_args, codex_args) = split_on_double_dash(&args);
    assert_eq!(
        loopr_args,
        vec!["--loopr-root".to_string(), "/tmp/root".to_string()]
    );
    assert_eq!(codex_args, vec!["--help".to_string()]);

    let args = vec!["--help".to_string()];
    let (loopr_args, codex_args) = split_on_double_dash(&args);
    assert_eq!(loopr_args, vec!["--help".to_string()]);
    assert!(codex_args.is_empty());
}

#[test]
fn test_extract_codex_passthrough_flags() {
    let (loopr_args, codex_args) =
        extract_codex_passthrough_flags(vec!["--help".to_string()], Vec::new());
    assert_eq!(loopr_args, vec!["--help".to_string()]);
    assert!(codex_args.is_empty());

    let (loopr_args, codex_args) = extract_codex_passthrough_flags(
        vec!["--codex".to_string(), "--help".to_string()],
        Vec::new(),
    );
    assert_eq!(loopr_args, vec!["--codex".to_string()]);
    assert_eq!(codex_args, vec!["--help".to_string()]);

    let (loopr_args, codex_args) = extract_codex_passthrough_flags(
        vec!["--codex".to_string(), "--version".to_string()],
        Vec::new(),
    );
    assert_eq!(loopr_args, vec!["--codex".to_string()]);
    assert_eq!(codex_args, vec!["--version".to_string()]);

    let (loopr_args, codex_args) = extract_codex_passthrough_flags(
        vec!["--codex=true".to_string(), "--help".to_string()],
        Vec::new(),
    );
    assert_eq!(loopr_args, vec!["--codex=true".to_string()]);
    assert_eq!(codex_args, vec!["--help".to_string()]);

    let (loopr_args, codex_args) =
        extract_codex_passthrough_flags(vec!["--codex".to_string()], vec!["--help".to_string()]);
    assert_eq!(loopr_args, vec!["--codex".to_string()]);
    assert_eq!(codex_args, vec!["--help".to_string()]);
}

#[test]
fn test_run_requires_codex_or_dry_run() {
    let (output, code) = run_loopr(&["run"]);
    assert_ne!(code, 0);
    assert!(output.contains("requires --codex or --dry-run"));
}

#[test]
fn test_run_dry_run_prints_steps() {
    let root = temp_dir("dry-run-print");
    let root_str = root.to_string_lossy();
    let (output, code) = run_loopr(&["run", "--dry-run", "--loopr-root", &root_str]);
    assert_eq!(code, 0);
    assert!(output.contains("Step: prd"));
}

#[test]
fn test_run_rejects_codex_and_dry_run() {
    let (output, code) = run_loopr(&["run", "--codex", "--dry-run"]);
    assert_ne!(code, 0);
    assert!(output.contains("mutually exclusive"));
}

#[test]
fn test_run_dry_run_ignores_confirm_and_agent_args() {
    let (output, code) = run_loopr(&["run", "--dry-run", "--confirm", "--", "--help"]);
    assert_eq!(code, 0);
    assert!(output.contains("Step: prd"));
}

#[test]
fn test_run_dry_run_rejects_invalid_step() {
    let (output, code) = run_loopr(&["run", "--dry-run", "--step", "nope"]);
    assert_ne!(code, 0);
    assert!(output.contains("unknown step: nope"));
}

#[test]
fn test_index_command_writes_docs_index() {
    let root = temp_dir("index-cmd");
    let root_str = root.to_string_lossy();
    let (_output, code) = run_loopr(&["init", "--no-agents", "--root", &root_str]);
    assert_eq!(code, 0);

    let (output, code) = run_loopr(&["index", "--loopr-root", &root_str]);
    assert_eq!(code, 0);
    assert!(output.contains("Docs index:"));
    assert!(root.join("loopr").join("state").join("docs-index.txt").exists());
}

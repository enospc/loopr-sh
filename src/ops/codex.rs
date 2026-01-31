use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use serde_json::json;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;
use time::macros::format_description;

use crate::ops::fs::ensure_dir;
use crate::ops::loopr_root::resolve_loopr_root;
use crate::version;
use crate::{LooprError, LooprResult};

pub struct CodexSession {
    pub repo_root: PathBuf,
    pub repo_id: String,
    pub log_path: PathBuf,
    pub meta_path: PathBuf,
    pub command: Vec<String>,
    pub started: OffsetDateTime,
}

pub struct CodexOptions {
    pub loopr_root: Option<PathBuf>,
    pub mode: CodexMode,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CodexMode {
    Interactive,
    Exec,
}

pub struct CodexRun {
    pub exit_code: i32,
    pub session: CodexSession,
    pub timed_out: bool,
    pub error_message: Option<String>,
}

pub fn run_codex(args: &[String], opts: &CodexOptions) -> LooprResult<CodexRun> {
    run_codex_internal(args, opts, None)
}

pub fn run_codex_with_timeout(
    args: &[String],
    opts: &CodexOptions,
    timeout: Duration,
) -> LooprResult<CodexRun> {
    run_codex_internal(args, opts, Some(timeout))
}

fn run_codex_internal(
    args: &[String],
    opts: &CodexOptions,
    timeout: Option<Duration>,
) -> LooprResult<CodexRun> {
    let cwd = std::env::current_dir()?;
    let (root, repo_id) = resolve_loopr_root(&cwd, opts.loopr_root.as_deref())?;

    let transcripts_dir = root
        .join("loopr")
        .join("state")
        .join("transcripts")
        .join(&repo_id);
    ensure_dir(&transcripts_dir, 0o755)?;

    let (log_path, meta_path) = new_session_paths(&transcripts_dir, OffsetDateTime::now_utc())?;

    let full_args = build_codex_args(args, opts.mode);
    let session = CodexSession {
        repo_root: root.clone(),
        repo_id: repo_id.clone(),
        log_path: log_path.clone(),
        meta_path: meta_path.clone(),
        command: std::iter::once("codex".to_string())
            .chain(full_args.iter().cloned())
            .collect(),
        started: OffsetDateTime::now_utc(),
    };

    let mut start_meta = serde_json::Map::new();
    start_meta.insert("event".to_string(), json!("start"));
    start_meta.insert("ts".to_string(), json!(format_rfc3339(session.started)?));
    start_meta.insert("cwd".to_string(), json!(cwd.display().to_string()));
    start_meta.insert("cmd".to_string(), json!(session.command.clone()));
    let log_name = session
        .log_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or_default();
    start_meta.insert("log".to_string(), json!(log_name));
    start_meta.insert("loopr_version".to_string(), json!(version::VERSION));
    start_meta.insert("loopr_commit".to_string(), json!(version::COMMIT));
    start_meta.insert("loopr_date".to_string(), json!(version::DATE));
    start_meta.insert("repo_root".to_string(), json!(root.display().to_string()));
    start_meta.insert("repo_id".to_string(), json!(repo_id));
    start_meta.insert(
        "codex_mode".to_string(),
        json!(match opts.mode {
            CodexMode::Interactive => "interactive",
            CodexMode::Exec => "exec",
        }),
    );

    let (commit, dirty) = git_info(&root);
    if !commit.is_empty() {
        start_meta.insert("git_commit".to_string(), json!(commit));
        if let Some(value) = dirty {
            start_meta.insert("git_dirty".to_string(), json!(value));
        }
    }

    write_meta(&meta_path, &serde_json::Value::Object(start_meta))?;

    let outcome = run_codex_with_logging_timeout(&log_path, &full_args, timeout, opts.mode)?;
    let end = OffsetDateTime::now_utc();
    let end_payload = json!({
        "event": "end",
        "ts": format_rfc3339(end)?,
        "exit_code": outcome.exit_code,
    });
    let _ = write_meta(&meta_path, &end_payload);

    Ok(CodexRun {
        exit_code: outcome.exit_code,
        session,
        timed_out: outcome.timed_out,
        error_message: outcome.error_message,
    })
}

fn build_codex_args(args: &[String], mode: CodexMode) -> Vec<String> {
    let mut full_args = Vec::with_capacity(args.len() + 1);
    if matches!(mode, CodexMode::Exec) {
        full_args.push("exec".to_string());
    }
    full_args.extend_from_slice(args);
    full_args
}

fn new_session_paths(dir: &Path, now: OffsetDateTime) -> LooprResult<(PathBuf, PathBuf)> {
    let ts = now
        .format(&format_description!(
            "[year][month][day]-[hour][minute][second]"
        ))
        .map_err(|err| LooprError::new(format!("format time: {}", err)))?;
    let prefix = format!("session-{}-", ts);

    for _ in 0..5 {
        let suffix = random_hex(6)?;
        let log_name = format!("{}{}.log", prefix, suffix);
        let log_path = dir.join(&log_name);
        let meta_path = dir.join(format!("{}{}.jsonl", prefix, suffix));

        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&log_path)
        {
            Ok(file) => {
                drop(file);
                if meta_path.exists() {
                    let _ = fs::remove_file(&log_path);
                    continue;
                }
                return Ok((log_path, meta_path));
            }
            Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(err) => {
                return Err(LooprError::new(format!(
                    "create session log {}: {}",
                    log_path.display(),
                    err
                )));
            }
        }
    }

    Err(LooprError::new("unable to allocate unique session paths"))
}

struct RunOutcome {
    exit_code: i32,
    timed_out: bool,
    error_message: Option<String>,
}

fn run_codex_with_logging_timeout(
    log_path: &Path,
    args: &[String],
    timeout: Option<Duration>,
    mode: CodexMode,
) -> LooprResult<RunOutcome> {
    if matches!(mode, CodexMode::Interactive) {
        return run_codex_interactive(log_path, args, timeout);
    }
    let file = File::create(log_path)
        .map_err(|err| LooprError::new(format!("create {}: {}", log_path.display(), err)))?;
    let file = Arc::new(Mutex::new(file));

    let mut cmd = Command::new("codex");
    cmd.args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    cmd.envs(std::env::vars());

    let mut child = match cmd.spawn() {
        Ok(child) => child,
        Err(err) => {
            return Ok(RunOutcome {
                exit_code: 1,
                timed_out: false,
                error_message: Some(format!("failed to start codex: {}", err)),
            });
        }
    };

    let stdout = match child.stdout.take() {
        Some(value) => value,
        None => {
            return Ok(RunOutcome {
                exit_code: 1,
                timed_out: false,
                error_message: Some("failed to capture codex stdout".to_string()),
            });
        }
    };
    let stderr = match child.stderr.take() {
        Some(value) => value,
        None => {
            return Ok(RunOutcome {
                exit_code: 1,
                timed_out: false,
                error_message: Some("failed to capture codex stderr".to_string()),
            });
        }
    };

    let stdout_handle = spawn_output_thread(stdout, Arc::clone(&file), StreamTarget::Stdout);
    let stderr_handle = spawn_output_thread(stderr, Arc::clone(&file), StreamTarget::Stderr);

    let start = Instant::now();
    let mut timed_out = false;
    let mut status: Option<ExitStatus> = None;

    loop {
        match child.try_wait() {
            Ok(Some(exit_status)) => {
                status = Some(exit_status);
                break;
            }
            Ok(None) => {}
            Err(err) => {
                return Ok(RunOutcome {
                    exit_code: 1,
                    timed_out: false,
                    error_message: Some(format!("wait for codex: {}", err)),
                });
            }
        }

        if let Some(limit) = timeout
            && start.elapsed() >= limit
        {
            timed_out = true;
            let _ = child.kill();
            let _ = child.wait();
            break;
        }
        std::thread::sleep(Duration::from_millis(200));
    }

    let exit_code = status.and_then(|value| value.code()).unwrap_or(1);

    let mut error_message = None;
    if let Some(err) = join_output(stdout_handle) {
        error_message = Some(err);
    }
    if let Some(err) = join_output(stderr_handle)
        && error_message.is_none()
    {
        error_message = Some(err);
    }

    Ok(RunOutcome {
        exit_code,
        timed_out,
        error_message,
    })
}

fn run_codex_interactive(
    log_path: &Path,
    args: &[String],
    timeout: Option<Duration>,
) -> LooprResult<RunOutcome> {
    let mut file = File::create(log_path)
        .map_err(|err| LooprError::new(format!("create {}: {}", log_path.display(), err)))?;
    file.write_all(b"[loopr] interactive codex session, output not captured\n")
        .map_err(|err| LooprError::new(format!("write {}: {}", log_path.display(), err)))?;

    let mut cmd = Command::new("codex");
    cmd.args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    cmd.envs(std::env::vars());

    let mut child = match cmd.spawn() {
        Ok(child) => child,
        Err(err) => {
            return Ok(RunOutcome {
                exit_code: 1,
                timed_out: false,
                error_message: Some(format!("failed to start codex: {}", err)),
            });
        }
    };

    let start = Instant::now();
    let mut timed_out = false;
    let mut status: Option<ExitStatus> = None;

    loop {
        match child.try_wait() {
            Ok(Some(exit_status)) => {
                status = Some(exit_status);
                break;
            }
            Ok(None) => {}
            Err(err) => {
                return Ok(RunOutcome {
                    exit_code: 1,
                    timed_out: false,
                    error_message: Some(format!("wait for codex: {}", err)),
                });
            }
        }

        if let Some(limit) = timeout
            && start.elapsed() >= limit
        {
            timed_out = true;
            let _ = child.kill();
            let _ = child.wait();
            break;
        }
        std::thread::sleep(Duration::from_millis(200));
    }

    let exit_code = status.and_then(|value| value.code()).unwrap_or(1);
    Ok(RunOutcome {
        exit_code,
        timed_out,
        error_message: None,
    })
}

enum StreamTarget {
    Stdout,
    Stderr,
}

fn spawn_output_thread<R: Read + Send + 'static>(
    mut reader: R,
    file: Arc<Mutex<File>>,
    target: StreamTarget,
) -> std::thread::JoinHandle<Option<String>> {
    std::thread::spawn(move || {
        let mut buf = [0u8; 8192];
        loop {
            let count = match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => n,
                Err(err) => return Some(err.to_string()),
            };

            if let Err(err) = write_shared(&file, &buf[..count]) {
                return Some(err);
            }
            let write_result = match target {
                StreamTarget::Stdout => write_stream(&mut std::io::stdout(), &buf[..count]),
                StreamTarget::Stderr => write_stream(&mut std::io::stderr(), &buf[..count]),
            };
            if let Err(err) = write_result {
                return Some(err);
            }
        }
        None
    })
}

fn write_shared(file: &Arc<Mutex<File>>, data: &[u8]) -> Result<(), String> {
    let mut guard = match file.lock() {
        Ok(value) => value,
        Err(_) => return Err("log mutex poisoned".to_string()),
    };
    guard.write_all(data).map_err(|err| err.to_string())?;
    guard.flush().map_err(|err| err.to_string())?;
    Ok(())
}

fn write_stream(writer: &mut dyn Write, data: &[u8]) -> Result<(), String> {
    writer.write_all(data).map_err(|err| err.to_string())?;
    writer.flush().map_err(|err| err.to_string())?;
    Ok(())
}

fn join_output(handle: std::thread::JoinHandle<Option<String>>) -> Option<String> {
    match handle.join() {
        Ok(value) => value,
        Err(_) => Some("output thread panicked".to_string()),
    }
}

fn write_meta(path: &Path, payload: &serde_json::Value) -> LooprResult<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|err| LooprError::new(format!("open {}: {}", path.display(), err)))?;
    let data = serde_json::to_vec(payload)
        .map_err(|err| LooprError::new(format!("serialize meta: {}", err)))?;
    file.write_all(&data)
        .map_err(|err| LooprError::new(format!("write meta {}: {}", path.display(), err)))?;
    file.write_all(b"\n")
        .map_err(|err| LooprError::new(format!("write meta {}: {}", path.display(), err)))?;
    Ok(())
}

fn git_info(root: &Path) -> (String, Option<bool>) {
    if Command::new("git")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_err()
    {
        return (String::new(), None);
    }

    let commit_out = Command::new("git")
        .arg("-C")
        .arg(root)
        .arg("rev-parse")
        .arg("HEAD")
        .output();
    let commit = match commit_out {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        _ => return (String::new(), None),
    };
    if commit.is_empty() {
        return (String::new(), None);
    }

    let status_out = Command::new("git")
        .arg("-C")
        .arg(root)
        .arg("status")
        .arg("--porcelain")
        .output();
    let dirty = match status_out {
        Ok(output) if output.status.success() => !output.stdout.is_empty(),
        _ => return (commit, None),
    };
    (commit, Some(dirty))
}

fn random_hex(bytes_len: usize) -> LooprResult<String> {
    let mut bytes = vec![0u8; bytes_len];
    getrandom::fill(&mut bytes).map_err(|err| LooprError::new(format!("random bytes: {}", err)))?;
    let mut out = String::with_capacity(bytes_len * 2);
    for byte in bytes {
        out.push(hex_digit(byte >> 4));
        out.push(hex_digit(byte & 0x0f));
    }
    Ok(out)
}

fn hex_digit(value: u8) -> char {
    match value {
        0..=9 => (b'0' + value) as char,
        10..=15 => (b'a' + (value - 10)) as char,
        _ => '0',
    }
}

fn format_rfc3339(time: OffsetDateTime) -> LooprResult<String> {
    time.format(&Rfc3339)
        .map_err(|err| LooprError::new(format!("format time: {}", err)))
}

#[cfg(test)]
mod tests {
    use crate::ops::codex::{CodexMode, build_codex_args, new_session_paths};
    use std::path::PathBuf;
    use time::macros::datetime;

    fn temp_dir(name: &str) -> PathBuf {
        let mut dir = std::env::temp_dir();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        dir.push(format!("loopr-test-{}-{}", name, nanos));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn test_new_session_paths_pairs_log_and_meta() {
        let dir = temp_dir("session-paths");
        let now = datetime!(2026-01-26 12:00:00 UTC);
        let (log_path, meta_path) = new_session_paths(&dir, now).unwrap();
        let log_base = log_path.file_name().unwrap().to_string_lossy();
        assert!(log_base.starts_with("session-20260126-120000-"));
        assert!(log_base.ends_with(".log"));
        let meta_base = meta_path.file_name().unwrap().to_string_lossy();
        assert!(meta_base.ends_with(".jsonl"));
        assert_eq!(
            log_base.trim_end_matches(".log"),
            meta_base.trim_end_matches(".jsonl")
        );
    }

    #[test]
    fn test_build_codex_args_exec_prefixes_subcommand() {
        let args = vec![
            "--cd".to_string(),
            "/repo".to_string(),
            "prompt".to_string(),
        ];
        let full = build_codex_args(&args, CodexMode::Exec);
        assert_eq!(full[0], "exec");
        assert_eq!(&full[1..], args.as_slice());
    }

    #[test]
    fn test_build_codex_args_interactive_passthrough() {
        let args = vec!["--cd".to_string(), "/repo".to_string()];
        let full = build_codex_args(&args, CodexMode::Interactive);
        assert_eq!(full, args);
    }
}

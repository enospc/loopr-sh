use std::fs;
use std::path::{Path, PathBuf};

use crate::ops::docs_index::write_docs_index;
use crate::ops::fs::{ensure_dir, write_file_atomic};
use crate::ops::nanoid::{OsRandom, RandomSource, generate_nanoid, repo_id_length};
use crate::{LooprError, LooprResult};

pub struct InitOptions {
    pub root: PathBuf,
    pub rand: Option<Box<dyn RandomSource>>,
    pub no_agents: bool,
}

pub struct InitReport {
    pub root: PathBuf,
    pub repo_id: String,
    pub repo_id_created: bool,
    pub transcripts_dir: PathBuf,
}

pub fn init(opts: InitOptions) -> LooprResult<InitReport> {
    let root = if opts.root.as_os_str().is_empty() {
        PathBuf::from(".")
    } else {
        opts.root
    };
    let abs_root = if root.is_absolute() {
        root
    } else {
        std::env::current_dir()?.join(root)
    };

    let loopr_dir = abs_root.join("loopr");
    let loopr_state_dir = loopr_dir.join("state");
    let repo_id_path = loopr_dir.join("repo-id");

    let mut report = InitReport {
        root: abs_root.clone(),
        repo_id: String::new(),
        repo_id_created: false,
        transcripts_dir: PathBuf::new(),
    };

    let existing_id = read_repo_id(&repo_id_path)?;
    ensure_dir(&loopr_dir, 0o755)?;
    ensure_loopr_gitignore(&loopr_dir)?;
    ensure_dir(&loopr_state_dir, 0o755)?;

    let repo_id = if let Some(value) = existing_id {
        value
    } else {
        let mut rng: Box<dyn RandomSource> = match opts.rand {
            Some(source) => source,
            None => Box::new(OsRandom),
        };
        let value = generate_nanoid(&mut *rng, repo_id_length())?;
        write_file_atomic(&repo_id_path, format!("{}\n", value).as_bytes(), 0o644)?;
        report.repo_id_created = true;
        value
    };

    let transcripts_dir = loopr_state_dir.join("transcripts").join(&repo_id);
    ensure_dir(&transcripts_dir, 0o755)?;

    report.repo_id = repo_id;
    report.transcripts_dir = transcripts_dir;

    if !opts.no_agents {
        ensure_agents_file(&abs_root)?;
    }
    write_docs_index(&abs_root)?;

    Ok(report)
}

fn ensure_agents_file(root: &Path) -> LooprResult<()> {
    let path = root.join("AGENTS.md");
    let stamp = now_rfc3339()?;
    let section = build_agents_section(&stamp);
    if path.exists() {
        let current = std::fs::read_to_string(&path)
            .map_err(|err| LooprError::new(format!("read {}: {}", path.display(), err)))?;
        if current.contains("[loopr: injected") {
            return Ok(());
        }
        let mut updated = current;
        if !updated.ends_with('\n') {
            updated.push('\n');
        }
        updated.push('\n');
        updated.push_str(&section);
        write_file_atomic(&path, updated.as_bytes(), 0o644)?;
        return Ok(());
    }

    let mut body = String::new();
    body.push_str("# AGENTS\n\n");
    body.push_str(&section);
    write_file_atomic(&path, body.as_bytes(), 0o644)?;
    Ok(())
}

fn build_agents_section(stamp: &str) -> String {
    format!(
        "## Loopr Harness Notes (injected by Loopr)\n[loopr: injected at {stamp}]\n\n- Always read AGENTS.md first.\n- Consult loopr/state/docs-index.txt for a compact map of docs.\n- Prefer explicit file reads over repo scanning.\n"
    )
}

fn now_rfc3339() -> LooprResult<String> {
    use time::OffsetDateTime;
    use time::format_description::well_known::Rfc3339;
    OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .map_err(|err| LooprError::new(format!("format time: {}", err)))
}

fn ensure_loopr_gitignore(loopr_dir: &Path) -> LooprResult<()> {
    let path = loopr_dir.join(".gitignore");
    if path.exists() {
        return Ok(());
    }
    let body = ["# Loopr runtime state is local-only.", "state/", ""].join("\n");
    write_file_atomic(&path, body.as_bytes(), 0o644)?;
    Ok(())
}

fn read_repo_id(path: &Path) -> LooprResult<Option<String>> {
    match fs::read_to_string(path) {
        Ok(value) => {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                return Err(LooprError::new(format!(
                    "repo-id is empty at {}",
                    path.display()
                )));
            }
            Ok(Some(trimmed.to_string()))
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(LooprError::new(format!("read {}: {}", path.display(), err))),
    }
}

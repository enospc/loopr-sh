use std::fs;
use std::path::{Path, PathBuf};

use crate::ops::fs::{ensure_dir, write_file_atomic};
use crate::ops::nanoid::{OsRandom, RandomSource, generate_nanoid, repo_id_length};
use crate::{LooprError, LooprResult};

pub struct InitOptions {
    pub root: PathBuf,
    pub rand: Option<Box<dyn RandomSource>>,
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

    Ok(report)
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

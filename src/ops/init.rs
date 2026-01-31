use std::fs;
use std::path::{Path, PathBuf};

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
    write_docs_index(&abs_root, &loopr_state_dir)?;

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

fn write_docs_index(root: &Path, state_dir: &Path) -> LooprResult<()> {
    let index_path = state_dir.join("docs-index.txt");
    let entries = collect_docs_entries(root)?;
    let mut body = String::new();
    for entry in entries {
        body.push_str(&entry);
        body.push('\n');
    }
    write_file_atomic(&index_path, body.as_bytes(), 0o644)?;
    Ok(())
}

fn collect_docs_entries(root: &Path) -> LooprResult<Vec<String>> {
    let mut files = Vec::new();
    push_if_exists(&mut files, root, "README.md");
    push_if_exists(&mut files, root, "AGENTS.md");
    push_if_exists(&mut files, root, "loopr/config");

    collect_dir_files(
        &mut files,
        root,
        root.join("docs"),
        &|path| is_markdown(path),
    )?;
    collect_dir_files(
        &mut files,
        root,
        root.join("specs"),
        &|path| is_markdown(path) || is_order_yaml(path),
    )?;

    files.sort();
    files.dedup();

    let mut entries = Vec::new();
    for file in files {
        let summary = summarize_file(root, &file)?;
        entries.push(format!("{} — {}", file, summary));
    }
    Ok(entries)
}

fn push_if_exists(files: &mut Vec<String>, root: &Path, rel: &str) {
    let path = root.join(rel);
    if path.is_file() {
        files.push(rel.to_string());
    }
}

fn collect_dir_files<F>(
    files: &mut Vec<String>,
    root: &Path,
    dir: PathBuf,
    filter: &F,
) -> LooprResult<()>
where
    F: Fn(&Path) -> bool,
{
    if !dir.exists() {
        return Ok(());
    }
    let entries = std::fs::read_dir(&dir)
        .map_err(|err| LooprError::new(format!("read dir {}: {}", dir.display(), err)))?;
    for entry in entries {
        let entry = entry.map_err(|err| LooprError::new(format!("read dir entry: {}", err)))?;
        let path = entry.path();
        let metadata = entry
            .metadata()
            .map_err(|err| LooprError::new(format!("metadata {}: {}", path.display(), err)))?;
        if metadata.is_dir() {
            collect_dir_files(files, root, path, filter)?;
        } else if metadata.is_file() && filter(&path) {
            if let Ok(rel) = path.strip_prefix(root) {
                files.push(rel.display().to_string());
            }
        }
    }
    Ok(())
}

fn is_markdown(path: &Path) -> bool {
    matches!(path.extension().and_then(|s| s.to_str()), Some("md"))
}

fn is_order_yaml(path: &Path) -> bool {
    let name = match path.file_name().and_then(|s| s.to_str()) {
        Some(value) => value,
        None => return false,
    };
    name.ends_with("-order.yaml")
}

fn summarize_file(root: &Path, rel: &str) -> LooprResult<String> {
    let path = root.join(rel);
    let metadata = std::fs::metadata(&path)
        .map_err(|err| LooprError::new(format!("metadata {}: {}", path.display(), err)))?;
    let max_size = 256 * 1024;
    if metadata.len() > max_size {
        return Ok("skipped (too large)".to_string());
    }
    let content = std::fs::read_to_string(&path)
        .map_err(|err| LooprError::new(format!("read {}: {}", path.display(), err)))?;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(stripped) = trimmed.strip_prefix('#') {
            let heading = stripped.trim_start_matches('#').trim();
            if !heading.is_empty() {
                return Ok(truncate_summary(heading));
            }
        }
        return Ok(truncate_summary(trimmed));
    }
    Ok("empty".to_string())
}

fn truncate_summary(value: &str) -> String {
    const LIMIT: usize = 120;
    if value.len() <= LIMIT {
        return value.to_string();
    }
    let mut out = value[..LIMIT].to_string();
    out.push_str("...");
    out
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

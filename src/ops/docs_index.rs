use std::path::{Path, PathBuf};

use crate::ops::fs::ensure_dir;
use crate::{LooprError, LooprResult};

pub fn write_docs_index(root: &Path) -> LooprResult<PathBuf> {
    let state_dir = root.join("loopr").join("state");
    ensure_dir(&state_dir, 0o755)?;
    let index_path = state_dir.join("docs-index.txt");
    let entries = collect_docs_entries(root)?;
    let mut body = String::new();
    for entry in entries {
        body.push_str(&entry);
        body.push('\n');
    }
    crate::ops::fs::write_file_atomic(&index_path, body.as_bytes(), 0o644)?;
    Ok(index_path)
}

fn collect_docs_entries(root: &Path) -> LooprResult<Vec<String>> {
    let mut files = Vec::new();
    push_if_exists(&mut files, root, "README.md");
    push_if_exists(&mut files, root, "AGENTS.md");
    push_if_exists(&mut files, root, "loopr/config");

    collect_dir_files(&mut files, root, root.join("docs"), &|path| is_markdown(path))?;
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
        let (size, summary) = summarize_file(root, &file)?;
        entries.push(format!("{}\t{}\t{}", file, size, summary));
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

fn summarize_file(root: &Path, rel: &str) -> LooprResult<(u64, String)> {
    let path = root.join(rel);
    let metadata = std::fs::metadata(&path)
        .map_err(|err| LooprError::new(format!("metadata {}: {}", path.display(), err)))?;
    let size = metadata.len();
    let max_size = 256 * 1024;
    if size > max_size {
        return Ok((size, "skipped (too large)".to_string()));
    }
    let content = match std::fs::read_to_string(&path) {
        Ok(value) => value,
        Err(_) => return Ok((size, "unreadable".to_string())),
    };
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(stripped) = trimmed.strip_prefix('#') {
            let heading = stripped.trim_start_matches('#').trim();
            if !heading.is_empty() {
                return Ok((size, truncate_summary(heading)));
            }
        }
        return Ok((size, truncate_summary(trimmed)));
    }
    Ok((size, "empty".to_string()))
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

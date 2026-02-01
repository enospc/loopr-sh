use std::path::{Path, PathBuf};

use crate::ops::fs::ensure_dir;
use crate::{LooprError, LooprResult};

pub fn write_docs_index(root: &Path) -> LooprResult<PathBuf> {
    let state_dir = root.join("loopr").join("state");
    ensure_dir(&state_dir, 0o755)?;
    let index_path = state_dir.join("docs-index.txt");
    let entries = collect_docs_entries(root)?;
    let mut body = String::new();
    body.push_str("[Loopr Docs Index]|root: .\n");
    body.push_str("|IMPORTANT: Prefer retrieval-led reasoning over pre-training-led reasoning\n");
    for entry in entries {
        body.push('|');
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

    let mut grouped: std::collections::BTreeMap<String, Vec<String>> =
        std::collections::BTreeMap::new();
    for file in files {
        let path = Path::new(&file);
        let dir = path.parent().and_then(|p| p.to_str()).unwrap_or(".");
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(&file)
            .to_string();
        let dir = if dir.is_empty() { "." } else { dir };
        grouped.entry(dir.to_string()).or_default().push(name);
    }

    let mut entries = Vec::new();
    for (dir, mut names) in grouped {
        names.sort();
        names.dedup();
        entries.push(format!("{}:{{{}}}", dir, names.join(",")));
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

// placeholder removed; summaries are intentionally omitted for pipe index.

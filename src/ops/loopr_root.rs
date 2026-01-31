use std::fs;
use std::path::{Path, PathBuf};

use crate::{LooprError, LooprResult};

pub fn find_loopr_root(start: &Path) -> LooprResult<(PathBuf, String)> {
    let mut current = start.to_path_buf();
    loop {
        let repo_id_path = current.join("loopr").join("repo-id");
        match fs::read_to_string(&repo_id_path) {
            Ok(data) => {
                let repo_id = data.trim();
                if repo_id.is_empty() {
                    return Err(LooprError::new(format!(
                        "repo-id is empty at {}",
                        repo_id_path.display()
                    )));
                }
                return Ok((current, repo_id.to_string()));
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
            Err(err) => {
                return Err(LooprError::new(format!(
                    "read {}: {}",
                    repo_id_path.display(),
                    err
                )));
            }
        }
        let parent = match current.parent() {
            Some(parent) => parent,
            None => break,
        };
        if parent == current {
            break;
        }
        current = parent.to_path_buf();
    }
    Err(LooprError::new(
        "unable to locate loopr/repo-id (run loopr init)",
    ))
}

pub fn resolve_loopr_root(
    start: &Path,
    override_root: Option<&Path>,
) -> LooprResult<(PathBuf, String)> {
    if let Some(root) = override_root {
        let root = if root.as_os_str().is_empty() {
            PathBuf::from(".")
        } else {
            root.to_path_buf()
        };
        return load_repo_id(&root);
    }
    find_loopr_root(start)
}

fn load_repo_id(root: &Path) -> LooprResult<(PathBuf, String)> {
    let abs_root = if root.is_absolute() {
        root.to_path_buf()
    } else {
        std::env::current_dir()?.join(root)
    };
    let repo_id_path = abs_root.join("loopr").join("repo-id");
    let data = match fs::read_to_string(&repo_id_path) {
        Ok(data) => data,
        Err(_) => {
            return Err(LooprError::new(format!(
                "unable to locate loopr/repo-id under {} (run loopr init)",
                abs_root.display()
            )));
        }
    };
    let repo_id = data.trim();
    if repo_id.is_empty() {
        return Err(LooprError::new(format!(
            "repo-id is empty at {}",
            repo_id_path.display()
        )));
    }
    Ok((abs_root, repo_id.to_string()))
}

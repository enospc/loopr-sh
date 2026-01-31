use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::{LooprError, LooprResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkStatusFile {
    pub version: i32,
    pub updated_at: String,
    pub items: HashMap<String, WorkItemStatus>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkItemStatus {
    pub key: String,
    pub item_type: WorkItemType,
    pub state: WorkItemState,
    pub attempts: u32,
    pub last_updated: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,
    #[serde(default)]
    pub pbt: bool,
    #[serde(default)]
    pub tests_written: bool,
    #[serde(default)]
    pub tests_validated: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_test: Option<TestRunResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WorkItemType {
    Task,
    Test,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WorkItemState {
    NotStarted,
    InProgress,
    Blocked,
    Error,
    Complete,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestRunResult {
    pub exit_code: i32,
    pub passed: bool,
    pub ran_at: String,
    pub phase: String,
}

pub fn load_work_status(path: &Path, now: &str) -> LooprResult<WorkStatusFile> {
    let data = match fs::read_to_string(path) {
        Ok(value) => value,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            return Ok(WorkStatusFile {
                version: 1,
                updated_at: now.to_string(),
                items: HashMap::new(),
            });
        }
        Err(err) => {
            return Err(LooprError::new(format!(
                "read {}: {}",
                path.display(),
                err
            )));
        }
    };

    serde_json::from_str(&data)
        .map_err(|err| LooprError::new(format!("parse {}: {}", path.display(), err)))
}

pub fn write_work_status(path: &Path, status: &WorkStatusFile) -> LooprResult<()> {
    let data = serde_json::to_vec_pretty(status)
        .map_err(|err| LooprError::new(format!("serialize {}: {}", path.display(), err)))?;
    let mut data = data;
    data.push(b'\n');
    crate::ops::fs::write_file_atomic(path, &data, 0o644)
}

pub fn ensure_item(
    status: &mut WorkStatusFile,
    key: &str,
    item_type: WorkItemType,
    now: &str,
) {
    status.items.entry(key.to_string()).or_insert_with(|| WorkItemStatus {
        key: key.to_string(),
        item_type,
        state: WorkItemState::NotStarted,
        attempts: 0,
        last_updated: now.to_string(),
        last_summary: None,
        last_error: None,
        pbt: false,
        tests_written: false,
        tests_validated: false,
        last_test: None,
    });
}

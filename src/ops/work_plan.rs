use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::{LooprError, LooprResult};

#[derive(Debug, Deserialize)]
pub struct TaskOrder {
    pub version: i64,
    pub tasks: Vec<TaskSpec>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TaskSpec {
    pub id: i64,
    pub key: String,
    pub title: String,
    pub file: String,
    #[serde(default)]
    pub depends_on: Vec<i64>,
}

#[derive(Debug, Deserialize)]
pub struct TestOrder {
    pub version: i64,
    pub tests: Vec<TestSpec>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TestSpec {
    pub id: i64,
    pub key: String,
    pub title: String,
    pub task_id: i64,
    pub file: String,
    #[serde(default)]
    pub depends_on: Vec<i64>,
    #[serde(default)]
    pub kind: Option<String>,
}

pub fn load_task_order(path: &Path) -> LooprResult<TaskOrder> {
    let data = fs::read_to_string(path)
        .map_err(|err| LooprError::new(format!("read {}: {}", path.display(), err)))?;
    serde_yaml::from_str(&data)
        .map_err(|err| LooprError::new(format!("parse {}: {}", path.display(), err)))
}

pub fn load_test_order(path: &Path) -> LooprResult<TestOrder> {
    let data = fs::read_to_string(path)
        .map_err(|err| LooprError::new(format!("read {}: {}", path.display(), err)))?;
    serde_yaml::from_str(&data)
        .map_err(|err| LooprError::new(format!("parse {}: {}", path.display(), err)))
}

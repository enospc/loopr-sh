use std::fs;
use std::path::Path;

use crate::{LooprError, LooprResult};

pub const LOOPR_STATUS_START: &str = "---LOOPR_STATUS---";
pub const LOOPR_STATUS_END: &str = "---END_LOOPR_STATUS---";

#[derive(Debug, Clone, Default)]
pub struct LooprStatus {
    pub status: String,
    pub exit_signal: bool,
    pub summary: String,
}

pub fn parse_loopr_status_from_log(path: &Path) -> LooprResult<(LooprStatus, bool)> {
    let data = fs::read_to_string(path)
        .map_err(|err| LooprError::new(format!("read {}: {}", path.display(), err)))?;
    Ok(parse_loopr_status(&data))
}

pub fn parse_loopr_status(log: &str) -> (LooprStatus, bool) {
    let idx = match log.rfind(LOOPR_STATUS_START) {
        Some(value) => value,
        None => return (LooprStatus::default(), false),
    };
    let mut segment = &log[idx + LOOPR_STATUS_START.len()..];
    if let Some(end) = segment.find(LOOPR_STATUS_END) {
        segment = &segment[..end];
    }

    let mut status = LooprStatus::default();
    for line in segment.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let mut parts = trimmed.splitn(2, ':');
        let key = match parts.next() {
            Some(value) => value.trim().to_uppercase(),
            None => continue,
        };
        let value = match parts.next() {
            Some(value) => value.trim(),
            None => continue,
        };
        match key.as_str() {
            "STATUS" => status.status = value.to_uppercase(),
            "EXIT_SIGNAL" => status.exit_signal = parse_bool(value),
            "SUMMARY" => status.summary = value.to_string(),
            _ => {}
        }
    }

    (status, true)
}

fn parse_bool(value: &str) -> bool {
    matches!(
        value.trim().to_lowercase().as_str(),
        "true" | "yes" | "1" | "y"
    )
}

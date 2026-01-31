use std::fs;
use std::path::Path;

use crate::{LooprError, LooprResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoopConfig {
    pub codex_timeout_minutes: i64,
    pub max_iterations: i64,
    pub max_missing_status: i64,
}

pub fn default_loop_config() -> LoopConfig {
    LoopConfig {
        codex_timeout_minutes: 15,
        max_iterations: 50,
        max_missing_status: 2,
    }
}

pub fn load_loop_config(path: &Path) -> LooprResult<LoopConfig> {
    let mut cfg = default_loop_config();
    let data = match fs::read_to_string(path) {
        Ok(value) => value,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(cfg),
        Err(err) => return Err(LooprError::new(format!("read {}: {}", path.display(), err))),
    };

    let mut line_no = 0;
    for line in data.lines() {
        line_no += 1;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let mut parts = trimmed.splitn(2, '=');
        let key = match parts.next() {
            Some(value) => value.trim(),
            None => {
                return Err(LooprError::new(format!(
                    "invalid config line {}: \"{}\" (expected KEY=VALUE)",
                    line_no, trimmed
                )));
            }
        };
        let value = match parts.next() {
            Some(value) => value.trim(),
            None => {
                return Err(LooprError::new(format!(
                    "invalid config line {}: \"{}\" (expected KEY=VALUE)",
                    line_no, trimmed
                )));
            }
        };
        let mut val = value;
        if let Some(hash_idx) = val.find('#') {
            val = val[..hash_idx].trim();
        }
        if val.is_empty() {
            return Err(LooprError::new(format!(
                "empty value for {} on line {}",
                key, line_no
            )));
        }
        apply_loop_config_value(&mut cfg, key, val, line_no)?;
    }

    Ok(cfg)
}

fn apply_loop_config_value(
    cfg: &mut LoopConfig,
    key: &str,
    val: &str,
    line_no: usize,
) -> LooprResult<()> {
    match key {
        "CODEX_TIMEOUT_MINUTES" => {
            set_loop_config_int(&mut cfg.codex_timeout_minutes, key, val, line_no, true)
        }
        "MAX_ITERATIONS" => set_loop_config_int(&mut cfg.max_iterations, key, val, line_no, false),
        "MAX_MISSING_STATUS" => {
            set_loop_config_int(&mut cfg.max_missing_status, key, val, line_no, true)
        }
        _ => Ok(()),
    }
}

fn set_loop_config_int(
    dst: &mut i64,
    key: &str,
    val: &str,
    line_no: usize,
    must_be_positive: bool,
) -> LooprResult<()> {
    let parsed: i64 = val.parse().map_err(|_| {
        LooprError::new(format!(
            "invalid int for {} on line {}: \"{}\"",
            key, line_no, val
        ))
    })?;
    if must_be_positive && parsed <= 0 {
        return Err(LooprError::new(format!(
            "{} must be > 0 on line {}",
            key, line_no
        )));
    }
    if !must_be_positive && parsed < 0 {
        return Err(LooprError::new(format!(
            "{} must be >= 0 on line {}",
            key, line_no
        )));
    }
    *dst = parsed;
    Ok(())
}

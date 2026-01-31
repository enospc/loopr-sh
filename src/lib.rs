pub mod cli;
pub mod ops;
pub mod version;

use std::fmt;

#[derive(Debug)]
pub struct LooprError {
    pub message: String,
}

impl LooprError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for LooprError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for LooprError {}

impl From<std::io::Error> for LooprError {
    fn from(err: std::io::Error) -> Self {
        Self::new(err.to_string())
    }
}

pub type LooprResult<T> = Result<T, LooprError>;

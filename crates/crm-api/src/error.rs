use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    BadRequest(String),
    Internal(String),
    Storage(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "not found: {msg}"),
            Self::BadRequest(msg) => write!(f, "bad request: {msg}"),
            Self::Internal(msg) => write!(f, "internal error: {msg}"),
            Self::Storage(msg) => write!(f, "storage error: {msg}"),
        }
    }
}

impl std::error::Error for ApiError {}

impl From<crm_core::CoreError> for ApiError {
    fn from(e: crm_core::CoreError) -> Self {
        Self::BadRequest(e.to_string())
    }
}

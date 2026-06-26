use std::fmt;

#[derive(Debug)]
pub enum CoreError {
    InvalidField(String),
    InvalidEntity(String),
    Validation(String),
    Relation(String),
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidField(msg) => write!(f, "invalid field: {msg}"),
            Self::InvalidEntity(msg) => write!(f, "invalid entity: {msg}"),
            Self::Validation(msg) => write!(f, "validation error: {msg}"),
            Self::Relation(msg) => write!(f, "relation error: {msg}"),
        }
    }
}

impl std::error::Error for CoreError {}

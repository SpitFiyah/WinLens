//! Error types for Privacy Debt Auditor

use std::fmt;
use std::io;

pub type Result<T> = std::result::Result<T, AuditError>;

#[derive(Debug)]
pub enum AuditError {
    /// IO error
    Io(io::Error),
    /// Database error
    Database(String),
    /// Regex parsing error
    RegexError(String),
    /// Serialization error
    SerializationError(String),
    /// Windows API error
    WindowsError(String),
    /// Missing resource
    NotFound(String),
    /// Permission denied
    PermissionDenied(String),
    /// Invalid configuration
    ConfigError(String),
    /// Generic error
    Other(String),
}

impl fmt::Display for AuditError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuditError::Io(e) => write!(f, "IO error: {}", e),
            AuditError::Database(e) => write!(f, "Database error: {}", e),
            AuditError::RegexError(e) => write!(f, "Regex error: {}", e),
            AuditError::SerializationError(e) => write!(f, "Serialization error: {}", e),
            AuditError::WindowsError(e) => write!(f, "Windows error: {}", e),
            AuditError::NotFound(e) => write!(f, "Not found: {}", e),
            AuditError::PermissionDenied(e) => write!(f, "Permission denied: {}", e),
            AuditError::ConfigError(e) => write!(f, "Config error: {}", e),
            AuditError::Other(e) => write!(f, "Error: {}", e),
        }
    }
}

impl std::error::Error for AuditError {}

impl From<io::Error> for AuditError {
    fn from(e: io::Error) -> Self {
        AuditError::Io(e)
    }
}

impl From<serde_json::Error> for AuditError {
    fn from(e: serde_json::Error) -> Self {
        AuditError::SerializationError(e.to_string())
    }
}

impl From<regex::Error> for AuditError {
    fn from(e: regex::Error) -> Self {
        AuditError::RegexError(e.to_string())
    }
}

impl From<std::env::VarError> for AuditError {
    fn from(e: std::env::VarError) -> Self {
        AuditError::ConfigError(format!("Environment variable error: {}", e))
    }
}

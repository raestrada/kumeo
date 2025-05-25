//! Runtime error handling

use std::fmt;
use thiserror::Error;

/// Standard result type for the runtime
pub type Result<T> = std::result::Result<T, RuntimeError>;

/// Runtime errors
#[derive(Debug, Error)]
pub enum RuntimeError {
    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    
    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    /// Resource not found error
    #[error("Resource not found: {0}")]
    NotFound(String),
    
    /// Permission error
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),
    
    /// Messaging error
    #[error("Messaging error: {0}")]
    Messaging(String),
    
    /// Error de recurso
    #[error("Resource error: {0}")]
    Resource(String),
    
    /// Timeout error
    #[error("Timeout: {0}")]
    Timeout(String),
    
    /// Generic error
    #[error("Error: {0}")]
    Other(String),
}

// Conversions from other error types
impl From<serde_json::Error> for RuntimeError {
    fn from(err: serde_json::Error) -> Self {
        RuntimeError::Serialization(err.to_string())
    }
}

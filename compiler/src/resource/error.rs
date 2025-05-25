use std::fmt;
use std::io;
use thiserror::Error;

/// Errors that can occur when working with resources
#[derive(Error, Debug)]
pub enum ResourceError {
    /// Resource not found
    #[error("Resource not found: {0}")]
    NotFound(String),
    
    /// Permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    /// Invalid URI
    #[error("Invalid URI: {0}")]
    InvalidUri(String),
    
    /// Unsupported scheme
    #[error("Unsupported scheme: {0}")]
    UnsupportedScheme(String),
    
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    /// HTTP error
    #[error("HTTP error: {0}")]
    Http(String),
    
    /// S3 error
    #[cfg(feature = "s3")]
    #[error("S3 error: {0}")]
    S3(String),
    
    /// Git error
    #[cfg(feature = "git")]
    #[error("Git error: {0}")]
    Git(String),
    
    /// Invalid UTF-8
    #[error("Invalid UTF-8: {0}")]
    InvalidUtf8(#[from] std::str::Utf8Error),
    
    /// Serialization/Deserialization error
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),
    
    /// Other errors
    #[error("Error: {0}")]
    Other(String),
}

// Implement From traits for common error types
impl From<reqwest::Error> for ResourceError {
    fn from(err: reqwest::Error) -> Self {
        ResourceError::Http(err.to_string())
    }
}

impl From<serde_json::Error> for ResourceError {
    fn from(err: serde_json::Error) -> Self {
        ResourceError::Serialization(err.to_string())
    }
}

impl From<serde_yaml::Error> for ResourceError {
    fn from(err: serde_yaml::Error) -> Self {
        ResourceError::Serialization(err.to_string())
    }
}

#[cfg(feature = "s3")]
impl From<aws_sdk_s3::Error> for ResourceError {
    fn from(err: aws_sdk_s3::Error) -> Self {
        ResourceError::S3(err.to_string())
    }
}

#[cfg(feature = "git")]
impl From<git2::Error> for ResourceError {
    fn from(err: git2::Error) -> Self {
        ResourceError::Git(err.to_string())
    }
}

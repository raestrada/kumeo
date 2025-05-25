use std::fmt;
use std::error::Error;

/// Errors that can occur when working with the Kumeo Runtime
#[derive(Debug)]
pub enum RuntimeError {
    /// Connection error with the runtime
    ConnectionError(String),
    
    /// Timeout while waiting for a response
    TimeoutError(String),
    
    /// Protocol error in communication with runtime
    ProtocolError(String),
    
    /// Resource not found
    ResourceNotFound(String),
    
    /// Permission denied
    PermissionDenied(String),
    
    /// Internal runtime error
    InternalError(String),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            RuntimeError::TimeoutError(msg) => write!(f, "Timeout error: {}", msg),
            RuntimeError::ProtocolError(msg) => write!(f, "Protocol error: {}", msg),
            RuntimeError::ResourceNotFound(msg) => write!(f, "Resource not found: {}", msg),
            RuntimeError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            RuntimeError::InternalError(msg) => write!(f, "Internal runtime error: {}", msg),
        }
    }
}

impl Error for RuntimeError {}

impl From<tonic::transport::Error> for RuntimeError {
    fn from(error: tonic::transport::Error) -> Self {
        RuntimeError::ConnectionError(format!("{}", error))
    }
}

impl From<tonic::Status> for RuntimeError {
    fn from(status: tonic::Status) -> Self {
        match status.code() {
            tonic::Code::DeadlineExceeded => RuntimeError::TimeoutError(status.message().to_string()),
            tonic::Code::NotFound => RuntimeError::ResourceNotFound(status.message().to_string()),
            tonic::Code::PermissionDenied => RuntimeError::PermissionDenied(status.message().to_string()),
            tonic::Code::InvalidArgument => RuntimeError::ProtocolError(status.message().to_string()),
            _ => RuntimeError::InternalError(format!("{}: {}", status.code(), status.message())),
        }
    }
}

//! Error types for the Kumeo parser.

use std::fmt;
use thiserror::Error;

/// Error type for parsing Kumeo DSL.
#[derive(Debug, Error)]
pub enum ParseError {
    /// Error from the Pest parser.
    #[error("Parse error: {0}")]
    PestError(#[from] Box<pest::error::Error<crate::parser::Rule>>),

    /// Semantic error in the DSL.
    #[error("Semantic error: {0}")]
    SemanticError(String),


    /// Generic error.
    #[error("Error: {0}")]
    Generic(String),
}

impl ParseError {
    /// Create a new semantic error.
    pub fn semantic<S: Into<String>>(msg: S) -> Self {
        Self::SemanticError(msg.into())
    }

    /// Create a new generic error.
    pub fn generic<S: Into<String>>(msg: S) -> Self {
        Self::Generic(msg.into())
    }
}

impl From<pest::error::Error<crate::parser::Rule>> for ParseError {
    fn from(err: pest::error::Error<crate::parser::Rule>) -> Self {
        Self::PestError(Box::new(err))
    }
}

/// Result type for parsing operations.
pub type ParseResult<T> = Result<T, ParseError>;

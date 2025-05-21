use thiserror::Error;

// Use thiserror's derive macro to implement Display automatically
#[derive(Error, Debug, Clone)]
pub enum KumeoError {
    #[error("Lexer error: {0}")]
    LexerError(String),
    
    #[error("Parser error at line {line}, column {column}: {message}")]
    ParserError {
        line: usize,
        column: usize,
        message: String,
    },
    
    #[error("Semantic error: {0}")]
    SemanticError(String),
    
    #[error("Type error: {0}")]
    TypeError(String),
    
    #[error("IO error: {0}")]
    IoError(String),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

// Implementation to convert std::io::Error to KumeoError
impl From<std::io::Error> for KumeoError {
    fn from(err: std::io::Error) -> Self {
        KumeoError::IoError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, KumeoError>;

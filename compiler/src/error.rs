use thiserror::Error;

// Use thiserror's derive macro to implement Display automatically
#[derive(Error, Debug)]
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
    IoError(#[from] std::io::Error),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

// No need for a manual Display implementation since thiserror generates it

pub type Result<T> = std::result::Result<T, KumeoError>;

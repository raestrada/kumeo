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
    
    #[error("Semantic errors: {0:?}")]
    SemanticErrors(Vec<String>),
    
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

// Implementation to convert ParseError to KumeoError
impl From<crate::parser::error::ParseError> for KumeoError {
    fn from(err: crate::parser::error::ParseError) -> Self {
        match err {
            crate::parser::error::ParseError::PestError(e) => {
                KumeoError::ParserError {
                    line: 0, // No tenemos acceso a la línea exacta desde Pest
                    column: 0, // No tenemos acceso a la columna exacta desde Pest
                    message: e.to_string(),
                }
            }
            crate::parser::error::ParseError::SemanticError(msg) => {
                KumeoError::ParserError {
                    line: 0,
                    column: 0,
                    message: msg,
                }
            }
            crate::parser::error::ParseError::Generic(msg) => {
                KumeoError::ParserError {
                    line: 0,
                    column: 0,
                    message: msg,
                }
            }
        }
    }
}

pub type Result<T> = std::result::Result<T, KumeoError>;
pub type SemanticResult = std::result::Result<(), Vec<String>>;

pub mod ast;
pub mod codegen;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod semantic;

// Import LALRPOP macro
use lalrpop_util::lalrpop_mod;

// Re-export main functionality
pub use crate::parser::parse;
pub use crate::ast::Program;
pub use crate::error::{KumeoError, Result};
pub use crate::semantic::SemanticAnalyzer;

// Process the LALRPOP grammar
lalrpop_mod!(pub kumeo);

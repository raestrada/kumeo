//! Kumeo Compiler
//! 
//! This crate provides the core compilation functionality for the Kumeo DSL.
//! It parses Kumeo source code and generates the corresponding target code.
//!
//! # Runtime Integration
//! The generated code communicates with the Kumeo runtime via Protobuf over Unix sockets
//! for all runtime operations including resource loading and message passing.

#![warn(missing_docs)]

pub mod ast;
pub mod codegen;
pub mod error;
pub mod lexer;
pub mod logging;
pub mod parser;
pub mod semantic;

// Import LALRPOP macro
use lalrpop_util::lalrpop_mod;

// Re-export main functionality
pub use crate::parser::parse;
pub use crate::ast::Program;
pub use crate::error::{KumeoError, Result};
pub use crate::semantic::SemanticAnalyzer;
pub use crate::logging::{init, LogFormat};

// Re-export tracing macros
pub use tracing::{debug, info, warn, error, trace};

// Process the LALRPOP grammar
lalrpop_mod!(pub kumeo);

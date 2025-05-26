//! Kumeo Compiler
//! 
//! This crate provides the core compilation functionality for the Kumeo DSL.
//! It parses Kumeo source code and generates the corresponding target code.
//!
//! # Architecture
//! - `ast`: Definiciones del Árbol de Sintaxis Abstracta (AST)
//! - `parser`: Análisis sintáctico del código fuente
//! - `semantic`: Análisis semántico y validación
//! - `codegen`: Generación de código
//! - `error`: Tipos de error y manejo de errores

#![warn(missing_docs)]

pub mod ast;
pub mod codegen;
pub mod error;
pub mod logging;
pub mod parser;
pub mod semantic;

// Re-export main functionality
pub use parser::parse;
pub use crate::ast::*;
pub use crate::error::{KumeoError, Result};
pub use crate::semantic::SemanticAnalyzer;
pub use crate::logging::{init, LogFormat};

// Re-export tracing macros
pub use tracing::{debug, info, warn, error, trace};

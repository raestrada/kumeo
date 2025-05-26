//! Módulo para el análisis semántico de programas Kumeo.

mod analyzer;

pub use analyzer::SemanticAnalyzer;

use crate::ast::Program;
use crate::error::{KumeoError, Result, SemanticResult};
use crate::parser;
use std::path::Path;

/// Realiza el análisis semántico de un programa Kumeo.
pub fn analyze_program(program: &Program) -> Result<()> {
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze_program(program)
}

/// Realiza el análisis semántico de un archivo Kumeo.
pub fn analyze_file<P: AsRef<Path>>(path: P) -> Result<()> {
    let content = std::fs::read_to_string(path.as_ref())?;
    let program = parser::parse(&content)?;
    analyze_program(&program)
}

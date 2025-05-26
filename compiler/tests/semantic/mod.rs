//! Pruebas para el análisis semántico de Kumeo DSL

mod workflow_validation;
mod subworkflow_validation;
mod agent_validation;

use kumeo_compiler::{parse, semantic::SemanticAnalyzer};

#[test]
fn test_empty_program_is_valid() {
    let program = parse("").expect("Debería parsear un programa vacío");
    let mut analyzer = SemanticAnalyzer::new();
    
    assert!(analyzer.analyze_program(&program).is_ok(), 
           "Un programa vacío debería ser válido");
}

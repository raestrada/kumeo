//! Pruebas para el parser de Kumeo DSL

mod workflow_tests;
mod subworkflow_tests;
mod error_handling_tests;

use kumeo_compiler::parser::parse;

#[test]
fn test_parse_empty_program() {
    let input = "";
    let program = parse(input).expect("Debería parsear un programa vacío");
    assert!(program.workflows.is_empty());
    assert!(program.subworkflows.is_empty());
}

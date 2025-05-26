use kumeo_compiler::{parse, semantic::SemanticAnalyzer};

#[test]
fn test_subworkflow_requires_name() {
    let input = "subworkflow {}";
    let program = parse(input).expect("Debería parsear incluso con error de sintaxis");
    let mut analyzer = SemanticAnalyzer::new();
    
    let result = analyzer.analyze_program(&program);
    assert!(result.is_err(), "Debería fallar por falta de nombre");
}

#[test]
fn test_subworkflow_can_have_agents() {
    let input = r#"
    subworkflow ValidSubworkflow {
        agent test { type = "llm" model = "llama3" }
    }
    "#;
    
    let program = parse(input).expect("Debería parsear");
    let mut analyzer = SemanticAnalyzer::new();
    
    let result = analyzer.analyze_program(&program);
    assert!(result.is_ok(), "Debería ser un subworkflow válido");
}

#[test]
fn test_duplicate_subworkflow_names() {
    let input = r#"
    subworkflow Duplicate { agent a { type = "llm" model = "llama3" } }
    subworkflow Duplicate { agent b { type = "llm" model = "llama3" } }
    "#;
    
    let program = parse(input).expect("Debería parsear");
    let mut analyzer = SemanticAnalyzer::new();
    
    let result = analyzer.analyze_program(&program);
    assert!(result.is_err(), "Debería fallar por nombres duplicados");
}

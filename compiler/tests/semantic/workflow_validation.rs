use kumeo_compiler::{parse, semantic::SemanticAnalyzer};

#[test]
fn test_workflow_requires_name() {
    let input = "workflow {}";
    let program = parse(input).expect("Debería parsear incluso con error de sintaxis");
    let mut analyzer = SemanticAnalyzer::new();
    
    let result = analyzer.analyze_program(&program);
    assert!(result.is_err(), "Debería fallar por falta de nombre");
}

#[test]
fn test_workflow_requires_source() {
    let input = r#"
    workflow TestWorkflow {
        // Sin source
        target output { type = "nats" topic = "out" }
    }
    "#;
    
    let program = parse(input).expect("Debería parsear");
    let mut analyzer = SemanticAnalyzer::new();
    
    let result = analyzer.analyze_program(&program);
    assert!(result.is_err(), "Debería fallar por falta de source");
}

#[test]
fn test_duplicate_workflow_names() {
    let input = r#"
    workflow Duplicate { source src { type = "nats" topic = "in" } }
    workflow Duplicate { source src { type = "nats" topic = "in" } }
    "#;
    
    let program = parse(input).expect("Debería parsear");
    let mut analyzer = SemanticAnalyzer::new();
    
    let result = analyzer.analyze_program(&program);
    assert!(result.is_err(), "Debería fallar por nombres duplicados");
}

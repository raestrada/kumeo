use kumeo_compiler::parser::parse;
use kumeo_compiler::error::KumeoError;

#[test]
fn test_missing_workflow_braces() {
    let input = r#"
    workflow MissingBraces
        agent test { type = "llm" model = "llama3" }
    "#;
    
    let result = parse(input);
    assert!(result.is_err(), "Debería fallar por llaves faltantes");
    
    assert!(result.is_err(), "Debería fallar por llaves faltantes");
    
    if let Err(err) = result {
        let err_str = err.to_string();
        assert!(err_str.contains("expected"), "Mensaje de error inesperado: {}", err_str);
    }
}

#[test]
fn test_invalid_agent_type() {
    let input = r#"
    workflow Test {
        source input { type = "nats" topic = "test" }
        agent invalid {
            type = "invalid_type"
            model = "test"
        }
    }
    "#;
    
    let result = parse(input);
    assert!(result.is_err(), "Debería fallar por tipo de agente inválido");
}

#[test]
fn test_duplicate_agent_id() {
    let input = r#"
    workflow Test {
        source input { type = "nats" topic = "test" }
        agent test { type = "llm" model = "llama3" }
        agent test { type = "ml" model = "classifier" }
    }
    "#;
    
    // Nota: Esta validación podría hacerse en el analizador semántico en lugar del parser
    let program = parse(input).expect("El parsing debería tener éxito");
    assert_eq!(program.workflows[0].agents.len(), 2); // El parser permite IDs duplicados
}

use kumeo_compiler::{parse, semantic::SemanticAnalyzer};

#[test]
fn test_llm_agent_requires_model() {
    let input = r#"
    workflow Test {
        source src { type = "nats" topic = "in" }
        agent llm_agent { type = "llm" }
    }
    "#;
    
    let program = parse(input).expect("Debería parsear");
    let mut analyzer = SemanticAnalyzer::new();
    
    let result = analyzer.analyze_program(&program);
    assert!(result.is_err(), "Debería fallar por falta de modelo en agente LLM");
}

#[test]
fn test_ml_agent_requires_model() {
    let input = r#"
    workflow Test {
        source src { type = "nats" topic = "in" }
        agent ml_agent { type = "ml" }
    }
    "#;
    
    let program = parse(input).expect("Debería parsear");
    let mut analyzer = SemanticAnalyzer::new();
    
    let result = analyzer.analyze_program(&program);
    assert!(result.is_err(), "Debería fallar por falta de modelo en agente ML");
}

#[test]
fn test_duplicate_agent_ids() {
    let input = r#"
    workflow Test {
        source src { type = "nats" topic = "in" }
        agent duplicate { type = "llm" model = "llama3" }
        agent duplicate { type = "ml" model = "classifier" }
    }
    "#;
    
    let program = parse(input).expect("Debería parsear");
    let mut analyzer = SemanticAnalyzer::new();
    
    let result = analyzer.analyze_program(&program);
    assert!(result.is_err(), "Debería fallar por IDs de agente duplicados");
}

#[test]
fn test_valid_llm_agent() {
    let input = r#"
    workflow Test {
        source src { type = "nats" topic = "in" }
        agent llm_agent { 
            type = "llm"
            model = "llama3"
            temperature = 0.7
            max_tokens = 1000
        }
    }
    "#;
    
    let program = parse(input).expect("Debería parsear");
    let mut analyzer = SemanticAnalyzer::new();
    
    let result = analyzer.analyze_program(&program);
    assert!(result.is_ok(), "Debería ser un agente LLM válido");
}

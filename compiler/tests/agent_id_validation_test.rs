use kumeo_compiler::{parse, SemanticAnalyzer};

#[test]
fn test_duplicate_agent_id_in_workflow() {
    // Create a program with duplicate agent IDs in a workflow
    let input = r#"
    workflow TestWorkflow {
        agents: [
            LLM(
                id: "agent1",
                engine: "llama3",
                prompt: "analyze text"
            ),
            MLModel(
                id: "agent1",
                model_path: "models/classifier"
            )
        ]
    }
    "#;

    let program = parse(input).expect("Failed to parse program");
    
    // Analyze the program for semantic errors
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    // Should detect duplicate agent ID
    assert!(result.is_err(), "Should have detected duplicate agent ID");
    
    // Check error message contains the duplicate ID
    let error_msg = format!("{:?}", analyzer.get_errors());
    assert!(
        error_msg.contains("agent1"), 
        "Error should mention duplicate agent ID: {}", error_msg
    );
}

#[test]
fn test_duplicate_agent_id_in_subworkflow_test() {
    // Create a program with duplicate agent IDs in a subworkflow
    let input = r#"
    subworkflow DataProcessor {
        input: ["data"]
        output: ["result"]
        agents: [
            LLM(
                id: "processor1",
                engine: "llama3",
                prompt: "analyze text"
            ),
            MLModel(
                id: "processor1",
                model_path: "models/classifier"
            )
        ]
    }
    "#;

    let program = parse(input).expect("Failed to parse program");
    
    // Analyze the program for semantic errors
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    // Should detect duplicate agent ID
    assert!(result.is_err(), "Should have detected duplicate agent ID in subworkflow");
    
    // Check error message contains the duplicate ID
    let error_msg = format!("{:?}", analyzer.get_errors());
    assert!(
        error_msg.contains("processor1"), 
        "Error should mention duplicate agent ID in subworkflow: {}", error_msg
    );
}

#[test]
fn test_unique_agent_ids_validation() {
    // Create a program with unique agent IDs (should pass validation)
    let input = r#"
    workflow ValidWorkflow {
        agents: [
            LLM(
                id: "text_processor",
                engine: "llama3",
                prompt: "analyze text"
            ),
            MLModel(
                id: "classifier",
                model_path: "models/classifier"
            ),
            BayesianNetwork(
                id: "risk_analyzer", 
                network_path: "models/risk.bn"
            )
        ]
    }
    "#;

    let program = parse(input).expect("Failed to parse program");
    
    // Analyze the program for semantic errors
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    // Should NOT detect any errors
    assert!(result.is_ok(), "Should NOT have detected any errors in valid workflow");
    
    let errors = analyzer.get_errors();
    assert!(errors.is_empty(), "Should have no errors");
}

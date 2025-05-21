use kumeo_compiler::{parse, SemanticAnalyzer};
use kumeo_compiler::ast::{Program, Workflow, Subworkflow, Integration, Mapping, Agent, AgentType, Argument, Value, PathExpr};
use std::collections::HashMap;

#[test]
fn test_duplicate_workflow_detection() {
    // Create a program with duplicate workflow names
    let input = r#"
    workflow TestWorkflow {
        agents: [
            LLM(
                id: "agent1",
                engine: "llama3"
            )
        ]
    }

    workflow TestWorkflow {
        agents: [
            MLModel(
                id: "agent2",
                model: "classifier"
            )
        ]
    }
    "#;

    let program = parse(input).expect("Failed to parse program");
    
    // Analyze the program for semantic errors
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    // Should detect the duplicate workflow name
    assert!(result.is_err(), "Should have detected duplicate workflow name");
    
    let errors = analyzer.get_errors();
    assert!(!errors.is_empty(), "Should have at least one error");
    
    // Error message should mention duplicate workflow
    let error_msg = format!("{:?}", errors[0]);
    assert!(error_msg.contains("Duplicate workflow"), 
            "Error should mention duplicate workflow: {}", error_msg);
}

#[test]
fn test_subworkflow_name_validation() {
    // Test validation of subworkflow names
    let input = r#"
    workflow MainWorkflow {
        agents: [
            LLM(
                id: "agent1",
                engine: "llama3"
            )
        ]
    }

    subworkflow ProcessData {
        input: ["data"]
        output: ["result"]
        agents: [
            MLModel(
                id: "model1",
                model: "classifier"
            )
        ]
    }

    subworkflow ProcessData {
        input: ["input"]
        output: ["output"]
        agents: [
            BayesianNetwork(
                id: "model2",
                network: "bayesian_model"
            )
        ]
    }
    "#;

    let program = parse(input).expect("Failed to parse program");
    
    // Analyze the program for semantic errors
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    // Should detect the duplicate subworkflow name
    assert!(result.is_err(), "Should have detected duplicate subworkflow name");
    
    let errors = analyzer.get_errors();
    assert!(!errors.is_empty(), "Should have at least one error");
    
    // Error messages should mention duplicate subworkflow
    let error_messages = errors.iter()
        .map(|e| format!("{:?}", e))
        .collect::<Vec<String>>()
        .join(", ");
    
    assert!(error_messages.contains("Duplicate subworkflow name"), 
            "Errors should mention duplicate subworkflow: {}", error_messages);
}

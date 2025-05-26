use kumeo_compiler::ast::{Source, Target, Agent, AgentType, Context, Value};
use kumeo_compiler::parser::parse;
use std::collections::HashMap;

// Inicializar el logger para los tests
fn init_logger() {
    use tracing_subscriber::{fmt, EnvFilter};
    use std::sync::Once;
    
    static INIT: Once = Once::new();
    
    INIT.call_once(|| {
        fmt()
            .with_env_filter(EnvFilter::from_default_env()
                .add_directive(tracing::Level::DEBUG.into()))
            .with_test_writer()
            .init();
    });
}

#[test]
fn test_simple_workflow_parsing() {
    init_logger();
    
    // Simple test input
    let test_input = r#"
    workflow SimpleWorkflow {
        source input {
            type = "nats"
            topic = "input-events"
        }
        
        target output {
            type = "nats"
            topic = "output-events"
        }
        
        agent LLM text_processor {
            engine = "ollama/llama3"
            prompt = "Analyze the following text: {{data}}"
        }
    }
    "#;
    
    // Parse the input
    let workflow = parse(test_input).expect("Should parse successfully");
    
    // Verify the AST structure
    assert_eq!(workflow.name, "SimpleWorkflow", "Workflow name should match");
    
    // Check source
    assert!(workflow.source.is_some(), "Workflow should have a source");
    let source = workflow.source.as_ref().unwrap();
    assert_eq!(source.r#type, "nats", "Source type should be nats");
    assert_eq!(
        source.config.get("topic").and_then(|v| v.as_str()),
        Some("input-events"),
        "Source topic should match"
    );
    
    // Check target
    assert!(workflow.target.is_some(), "Workflow should have a target");
    let target = workflow.target.as_ref().unwrap();
    assert_eq!(target.r#type, "nats", "Target type should be nats");
    assert_eq!(
        target.config.get("topic").and_then(|v| v.as_str()),
        Some("output-events"),
        "Target topic should match"
    );
    
    // Check agents
    assert_eq!(workflow.agents.len(), 1, "Should have 1 agent");
    
    let agent = &workflow.agents[0];
    assert_eq!(agent.agent_type, AgentType::LLM, "Agent should be of type LLM");
    assert_eq!(agent.id, "text_processor", "Agent ID should match");
    
    // Check agent configuration
    assert_eq!(
        agent.config.get("engine").and_then(|v| v.as_str()),
        Some("ollama/llama3"),
        "Engine should match"
    );
    
    assert_eq!(
        agent.config.get("prompt").and_then(|v| v.as_str()),
        Some("Analyze the following text: {{data}}"),
        "Prompt should match"
    );
}

#[test]
fn test_workflow_with_multi_agent() {
    init_logger();
    
    // Test input with multiple agents
    let test_input = r#"
    workflow MultiAgentWorkflow {
        source input {
            type = "nats"
            topic = "data-input"
        }
        
        agent LLM summarizer {
            engine = "mistral"
        }
        
        agent MLModel classifier {
            model_path = "models/classifier"
        }
    }
    "#;
    
    // Parse the input
    let workflow = parse(test_input).expect("Should parse successfully");
    
    // Verify the AST structure
    assert_eq!(workflow.name, "MultiAgentWorkflow", "Workflow name should match");
    
    // Check source
    assert!(workflow.source.is_some(), "Workflow should have a source");
    let source = workflow.source.as_ref().unwrap();
    assert_eq!(source.r#type, "nats", "Source type should be nats");
    assert_eq!(
        source.config.get("topic").and_then(|v| v.as_str()),
        Some("data-input"),
        "Source topic should match"
    );
    
    // Check agents
    assert_eq!(workflow.agents.len(), 2, "Should have 2 agents");
    
    // Verify first agent (LLM)
    let llm_agent = workflow.agents.iter()
        .find(|a| a.id == "summarizer")
        .expect("Should find LLM agent");
    
    assert_eq!(
        llm_agent.agent_type, 
        AgentType::LLM, 
        "First agent should be of type LLM"
    );
    
    assert_eq!(
        llm_agent.config.get("engine").and_then(|v| v.as_str()),
        Some("mistral"),
        "LLM engine should be mistral"
    );
    
    // Verify second agent (MLModel)
    let ml_agent = workflow.agents.iter()
        .find(|a| a.id == "classifier")
        .expect("Should find MLModel agent");
    
    assert_eq!(
        ml_agent.agent_type,
        AgentType::MLModel,
        "Second agent should be of type MLModel"
    );
    
    assert_eq!(
        ml_agent.config.get("model_path").and_then(|v| v.as_str()),
        Some("models/classifier"),
        "Model path should match"
    );
}

use kumeo_compiler::ast::{Source, Target, AgentType, Argument, Value};
use kumeo_compiler::parse;

#[test]
fn test_simple_workflow_parsing() {
    // Simple test input
    let test_input = r#"
        workflow SimpleWorkflow {
            source: NATS("input-events")
            target: NATS("output-events")
            
            agents: [
                LLM(
                    id: "text_processor",
                    engine: "ollama/llama3",
                    prompt: "Analyze the following text: {{data}}"
                )
            ]
        }
    "#;
    
    // Parse the input
    let program = parse(test_input).expect("Should parse successfully");
    
    // Verify the AST structure
    assert_eq!(program.workflows.len(), 1, "Should have 1 workflow");
    
    let workflow = &program.workflows[0];
    assert_eq!(workflow.name, "SimpleWorkflow", "Workflow name should match");
    
    // Check source
    assert!(workflow.source.is_some(), "Workflow should have a source");
    if let Some(Source::NATS(topic, _)) = &workflow.source {
        assert_eq!(topic, "input-events", "Source topic should match");
    } else {
        panic!("Source should be NATS");
    }
    
    // Check target
    assert!(workflow.target.is_some(), "Workflow should have a target");
    if let Some(Target::NATS(topic, _)) = &workflow.target {
        assert_eq!(topic, "output-events", "Target topic should match");
    } else {
        panic!("Target should be NATS");
    }
    
    // Check agents
    assert_eq!(workflow.agents.len(), 1, "Should have 1 agent");
    
    let agent = &workflow.agents[0];
    match agent.agent_type {
        AgentType::LLM => { /* Expected */ },
        _ => panic!("Agent should be of type LLM")
    };
    assert_eq!(agent.id, Some("text_processor".to_string()), "Agent ID should match");
    
    // Check agent configuration
    let mut engine_found = false;
    let mut prompt_found = false;
    
    for arg in &agent.config {
        match arg {
            Argument::Named(name, Value::String(value)) => {
                if name == "engine" {
                    assert_eq!(value, "ollama/llama3", "Engine should match");
                    engine_found = true;
                } else if name == "prompt" {
                    assert_eq!(value, "Analyze the following text: {{data}}", "Prompt should match");
                    prompt_found = true;
                }
            }
            _ => {}
        }
    }
    
    assert!(engine_found, "Agent should have engine configuration");
    assert!(prompt_found, "Agent should have prompt configuration");
}

#[test]
fn test_workflow_with_multi_agent() {
    // Test input with multiple agents
    let test_input = r#"
        workflow MultiAgentWorkflow {
            source: NATS("data-input")
            
            agents: [
                LLM(
                    id: "summarizer",
                    engine: "mistral"
                ),
                MLModel(
                    id: "classifier",
                    model_path: "models/classifier"
                )
            ]
        }
    "#;
    
    // Parse the input
    let program = parse(test_input).expect("Should parse successfully");
    
    // Verify the AST structure
    assert_eq!(program.workflows.len(), 1, "Should have 1 workflow");
    
    let workflow = &program.workflows[0];
    assert_eq!(workflow.name, "MultiAgentWorkflow", "Workflow name should match");
    
    // Check agents
    assert_eq!(workflow.agents.len(), 2, "Should have 2 agents");
    
    // First agent
    let agent1 = &workflow.agents[0];
    match agent1.agent_type {
        AgentType::LLM => { /* Expected */ },
        _ => panic!("First agent should be of type LLM")
    };
    assert_eq!(agent1.id, Some("summarizer".to_string()), "First agent ID should match");
    
    // Second agent
    let agent2 = &workflow.agents[1];
    match agent2.agent_type {
        AgentType::MLModel => { /* Expected */ },
        _ => panic!("Second agent should be of type MLModel")
    };
    assert_eq!(agent2.id, Some("classifier".to_string()), "Second agent ID should match");
}

use kumeo_compiler::ast::{Agent, AgentType, Argument, Program, Subworkflow, Value, Workflow};
use kumeo_compiler::semantic::SemanticAnalyzer;

#[test]
fn test_workflow_missing_source() {
    let mut program = Program::new();
    
    // Create a workflow without a source
    let workflow = Workflow {
        name: "workflow1".to_string(),
        source: None, // Missing source
        target: Some(kumeo_compiler::ast::Target::NATS("topic".to_string(), None)),
        context: None,
        preprocessors: None,
        agents: vec![
            Agent {
                id: Some("agent1".to_string()),
                agent_type: AgentType::LLM,
                config: vec![
                    Argument::Named("engine".to_string(), Value::String("gpt-4".to_string())),
                    Argument::Named("prompt".to_string(), Value::String("Hello".to_string())),
                ],
            }
        ],
        monitor: None,
        deployment: None,
    };
    
    program.workflows.push(workflow);
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    // Verificar que el análisis falló como se esperaba
    assert!(result.is_err(), "Se esperaba que el análisis fallara debido a la falta de source");
    
    let errors = analyzer.get_errors();
    assert!(!errors.is_empty());
    assert!(errors.iter().any(|e| e.to_string().contains("missing a source configuration")));
}

#[test]
fn test_workflow_missing_target() {
    let mut program = Program::new();
    
    // Create a workflow without a target
    let workflow = Workflow {
        name: "workflow1".to_string(),
        source: Some(kumeo_compiler::ast::Source::NATS("topic".to_string(), None)),
        target: None, // Missing target
        context: None,
        preprocessors: None,
        agents: vec![
            Agent {
                id: Some("agent1".to_string()),
                agent_type: AgentType::LLM,
                config: vec![
                    Argument::Named("engine".to_string(), Value::String("gpt-4".to_string())),
                    Argument::Named("prompt".to_string(), Value::String("Hello".to_string())),
                ],
            }
        ],
        monitor: None,
        deployment: None,
    };
    
    program.workflows.push(workflow);
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    // Verificar que el análisis falló como se esperaba
    assert!(result.is_err(), "Se esperaba que el análisis fallara debido a la falta de target");
    
    let errors = analyzer.get_errors();
    assert!(!errors.is_empty());
    assert!(errors.iter().any(|e| e.to_string().contains("missing a target configuration")));
}

#[test]
fn test_subworkflow_missing_input() {
    let mut program = Program::new();
    
    // Create a subworkflow without inputs
    let subworkflow = Subworkflow {
        name: "subworkflow1".to_string(),
        input: None, // Missing input
        output: Some(vec!["output1".to_string()]),
        context: None,
        agents: vec![
            Agent {
                id: Some("agent1".to_string()),
                agent_type: AgentType::LLM,
                config: vec![
                    Argument::Named("engine".to_string(), Value::String("gpt-4".to_string())),
                    Argument::Named("prompt".to_string(), Value::String("Hello".to_string())),
                ],
            }
        ],
    };
    
    program.subworkflows.push(subworkflow);
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    // Verificar que el análisis falló como se esperaba
    assert!(result.is_err(), "Se esperaba que el análisis fallara debido a la falta de definiciones de input");
    
    let errors = analyzer.get_errors();
    assert!(!errors.is_empty());
    assert!(errors.iter().any(|e| e.to_string().contains("missing input definitions")));
}

#[test]
fn test_subworkflow_missing_output() {
    let mut program = Program::new();
    
    // Create a subworkflow without outputs
    let subworkflow = Subworkflow {
        name: "subworkflow1".to_string(),
        input: Some(vec!["input1".to_string()]),
        output: None, // Missing output
        context: None,
        agents: vec![
            Agent {
                id: Some("agent1".to_string()),
                agent_type: AgentType::LLM,
                config: vec![
                    Argument::Named("engine".to_string(), Value::String("gpt-4".to_string())),
                    Argument::Named("prompt".to_string(), Value::String("Hello".to_string())),
                ],
            }
        ],
    };
    
    program.subworkflows.push(subworkflow);
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    // Verificar que el análisis falló como se esperaba
    assert!(result.is_err(), "Se esperaba que el análisis fallara debido a la falta de definiciones de output");
    
    let errors = analyzer.get_errors();
    assert!(!errors.is_empty());
    assert!(errors.iter().any(|e| e.to_string().contains("missing output definitions")));
}

#[test]
fn test_agent_missing_required_config() {
    let mut program = Program::new();
    
    // Create a workflow with an LLM agent that's missing a required parameter ("prompt")
    let workflow = Workflow {
        name: "workflow1".to_string(),
        source: Some(kumeo_compiler::ast::Source::NATS("topic".to_string(), None)),
        target: Some(kumeo_compiler::ast::Target::NATS("topic".to_string(), None)),
        context: None,
        preprocessors: None,
        agents: vec![
            Agent {
                id: Some("agent1".to_string()),
                agent_type: AgentType::LLM,
                config: vec![
                    Argument::Named(
                        "engine".to_string(), 
                        Value::String("gpt-4".to_string())
                    ),
                    // Missing "prompt" parameter
                ],
            }
        ],
        monitor: None,
        deployment: None,
    };
    
    program.workflows.push(workflow);
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    // Verificar que el análisis falló como se esperaba
    assert!(result.is_err(), "Se esperaba que el análisis fallara debido a la falta del parámetro 'prompt'");
    
    let errors = analyzer.get_errors();
    assert!(!errors.is_empty());
    assert!(errors.iter().any(|e| e.to_string().contains("missing required parameter 'prompt'")));
}

#[test]
fn test_preprocessor_agent_missing_required_config() {
    let mut program = Program::new();
    
    // Create a workflow with a preprocessor of type MLModel that's missing required parameter
    let workflow = Workflow {
        name: "workflow1".to_string(),
        source: Some(kumeo_compiler::ast::Source::NATS("topic".to_string(), None)),
        target: Some(kumeo_compiler::ast::Target::NATS("topic".to_string(), None)),
        context: None,
        preprocessors: Some(vec![
            Agent {
                id: Some("preprocessor1".to_string()),
                agent_type: AgentType::MLModel,
                config: vec![
                    // Missing "model_path" parameter
                ],
            }
        ]),
        agents: vec![
            Agent {
                id: Some("agent1".to_string()),
                agent_type: AgentType::LLM,
                config: vec![
                    Argument::Named("engine".to_string(), Value::String("gpt-4".to_string())),
                    Argument::Named("prompt".to_string(), Value::String("Hello".to_string())),
                ],
            }
        ],
        monitor: None,
        deployment: None,
    };
    
    program.workflows.push(workflow);
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    // Verificar que el análisis falló como se esperaba
    assert!(result.is_err(), "Se esperaba que el análisis fallara debido a la falta del parámetro 'model_path'");
    
    let errors = analyzer.get_errors();
    assert!(!errors.is_empty());
    assert!(errors.iter().any(|e| e.to_string().contains("missing required parameter 'model_path'")));
}

#[test]
fn test_subworkflow_agent_missing_required_config() {
    let mut program = Program::new();
    
    // Create a subworkflow with an agent of type BayesianNetwork missing required parameter
    let subworkflow = Subworkflow {
        name: "subworkflow1".to_string(),
        input: Some(vec!["input1".to_string()]),
        output: Some(vec!["output1".to_string()]),
        context: None,
        agents: vec![
            Agent {
                id: Some("agent1".to_string()),
                agent_type: AgentType::BayesianNetwork,
                config: vec![
                    // Missing "network_path" parameter
                ],
            }
        ],
    };
    
    program.subworkflows.push(subworkflow);
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    // Verificar que el análisis falló como se esperaba
    assert!(result.is_err(), "Se esperaba que el análisis fallara debido a la falta del parámetro 'network_path'");
    
    let errors = analyzer.get_errors();
    assert!(!errors.is_empty());
    assert!(errors.iter().any(|e| e.to_string().contains("missing required parameter 'network_path'")));
}

#[test]
fn test_valid_workflow_and_subworkflow() {
    let mut program = Program::new();
    
    // Create a valid workflow with source, target, and proper agent config
    let workflow = Workflow {
        name: "workflow1".to_string(),
        source: Some(kumeo_compiler::ast::Source::NATS("topic".to_string(), None)),
        target: Some(kumeo_compiler::ast::Target::NATS("topic".to_string(), None)),
        context: None,
        preprocessors: None,
        agents: vec![
            Agent {
                id: Some("agent1".to_string()),
                agent_type: AgentType::LLM,
                config: vec![
                    Argument::Named("engine".to_string(), Value::String("gpt-4".to_string())),
                    Argument::Named("prompt".to_string(), Value::String("Hello".to_string())),
                ],
            }
        ],
        monitor: None,
        deployment: None,
    };
    
    // Create a valid subworkflow with inputs, outputs, and proper agent config
    let subworkflow = Subworkflow {
        name: "subworkflow1".to_string(),
        input: Some(vec!["input1".to_string()]),
        output: Some(vec!["output1".to_string()]),
        context: None,
        agents: vec![
            Agent {
                id: Some("agent1".to_string()),
                agent_type: AgentType::MLModel,
                config: vec![
                    Argument::Named("model_path".to_string(), Value::String("/path/to/model".to_string())),
                ],
            }
        ],
    };
    
    program.workflows.push(workflow);
    program.subworkflows.push(subworkflow);
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    let errors = analyzer.get_errors();
    assert!(errors.is_empty(), "Expected no errors but got: {:?}", errors);
}

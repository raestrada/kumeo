use std::fs;
use std::path::{Path, PathBuf};
use kumeo_compiler::ast::{Agent, AgentType, Workflow, Value, Argument};
use kumeo_compiler::codegen::rust_generator::RustGenerator;

#[test]
fn test_rust_generator_initialization() {
    // Create a temporary directory for templates and output
    let temp_dir = tempfile::tempdir().unwrap();
    let template_root = temp_dir.path().join("templates");
    let output_dir = temp_dir.path().join("output");
    
    // Create template directories
    fs::create_dir_all(&template_root.join("rust/agents")).unwrap();
    
    // Create a simple template for testing
    let template_content = "// Template for {{agent_id}} agent\npub struct {{agent_id}}Agent {\n    // Configuration\n}";
    fs::write(template_root.join("rust/agents/llm.rs.tmpl"), template_content).unwrap();
    
    // Initialize the Rust generator
    let rust_generator = RustGenerator::new(&template_root, &output_dir);
    assert!(rust_generator.is_ok(), "Failed to initialize Rust generator");
}

#[test]
fn test_llm_agent_generation() {
    // Create a temporary directory for templates and output
    let temp_dir = tempfile::tempdir().unwrap();
    let template_root = temp_dir.path().join("templates");
    let output_dir = temp_dir.path().join("output");
    
    // Create template directories
    fs::create_dir_all(&template_root.join("rust/agents")).unwrap();
    
    // Create a simple template for testing
    let template_content = "// Template for {{agent_id}} agent\npub struct {{agent_id}}Agent {\n    engine: String, // Using {{engine}}\n    prompt: String, // Using {{prompt}}\n}\n\n// Values: engine={{engine}}, prompt={{prompt}}";
    fs::write(template_root.join("rust/agents/llm.rs.tmpl"), template_content).unwrap();
    
    // Initialize the Rust generator
    let mut rust_generator = RustGenerator::new(&template_root, &output_dir).unwrap();
    
    // Create a test agent
    let agent = Agent {
        id: Some("TestLLM".to_string()),
        agent_type: AgentType::LLM,
        config: vec![
            Argument::Named("engine".to_string(), Value::String("gpt-4".to_string())),
            Argument::Named("prompt".to_string(), Value::String("Hello, world!".to_string())),
        ],
    };
    
    // Create a test workflow
    let workflow = Workflow {
        name: "TestWorkflow".to_string(),
        source: None,
        target: None,
        context: None,
        preprocessors: None,
        agents: vec![],
        monitor: None,
        deployment: None,
    };
    
    // Generate code for the LLM agent
    let result = rust_generator.generate_llm_agent(&agent, &workflow);
    assert!(result.is_ok(), "Failed to generate LLM agent code");
    
    // Check the generated file
    let output_file = result.unwrap();
    assert!(output_file.exists(), "Output file was not created");
    
    // Check the content of the file
    let file_content = fs::read_to_string(&output_file).unwrap();
    assert!(file_content.contains("TestLLM"), "Agent ID not found in generated code");
    assert!(file_content.contains("gpt-4"), "Engine parameter not found in generated code");
    assert!(file_content.contains("Hello, world!"), "Prompt parameter not found in generated code");
}

#[test]
fn test_router_agent_generation() {
    // Create a temporary directory for templates and output
    let temp_dir = tempfile::tempdir().unwrap();
    let template_root = temp_dir.path().join("templates");
    let output_dir = temp_dir.path().join("output");
    
    // Create template directories
    fs::create_dir_all(&template_root.join("rust/agents")).unwrap();
    
    // Create a simple template for testing
    let template_content = "// Template for {{agent_id}} agent\npub struct {{agent_id}}Agent {\n    routing_rules: String,\n}";
    fs::write(template_root.join("rust/agents/router.rs.tmpl"), template_content).unwrap();
    
    // Initialize the Rust generator
    let mut rust_generator = RustGenerator::new(&template_root, &output_dir).unwrap();
    
    // Create a test agent
    let agent = Agent {
        id: Some("TestRouter".to_string()),
        agent_type: AgentType::Router,
        config: vec![
            Argument::Named("routing_rules".to_string(), Value::String("{\"type1\": \"topic1\", \"type2\": \"topic2\"}".to_string())),
        ],
    };
    
    // Create a test workflow
    let workflow = Workflow {
        name: "TestWorkflow".to_string(),
        source: None,
        target: None,
        context: None,
        preprocessors: None,
        agents: vec![],
        monitor: None,
        deployment: None,
    };
    
    // Generate code for the Router agent
    let result = rust_generator.generate_router_agent(&agent, &workflow);
    assert!(result.is_ok(), "Failed to generate Router agent code");
    
    // Check the generated file
    let output_file = result.unwrap();
    assert!(output_file.exists(), "Output file was not created");
    
    // Check the content of the file
    let file_content = fs::read_to_string(&output_file).unwrap();
    assert!(file_content.contains("TestRouter"), "Agent ID not found in generated code");
    assert!(file_content.contains("routing_rules"), "Routing rules parameter not found in generated code");
}

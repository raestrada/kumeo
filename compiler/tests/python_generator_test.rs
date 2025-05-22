use std::fs;
use std::path::PathBuf;
use kumeo_compiler::ast::{Agent, AgentType, Workflow, Value, Argument};
use kumeo_compiler::codegen::python_generator::PythonGenerator;

#[test]
fn test_python_generator_initialization() {
    // Create a temporary directory for templates and output
    let temp_dir = tempfile::tempdir().unwrap();
    let template_root = temp_dir.path().join("templates");
    let output_dir = temp_dir.path().join("output");
    
    // Create template directories
    fs::create_dir_all(&template_root.join("python/agents")).unwrap();
    
    // Create a simple template for testing
    let template_content = "# Template for {{agent_id}} agent\nclass {{agent_id}}Agent:\n    # Configuration\n    pass";
    fs::write(template_root.join("python/agents/ml_model.py.tmpl"), template_content).unwrap();
    
    // Initialize the Python generator
    let python_generator = PythonGenerator::new(&template_root, &output_dir);
    assert!(python_generator.is_ok(), "Failed to initialize Python generator");
}

#[test]
fn test_ml_model_agent_generation() {
    // Create a temporary directory for templates and output
    let temp_dir = tempfile::tempdir().unwrap();
    let template_root = temp_dir.path().join("templates");
    let output_dir = temp_dir.path().join("output");
    
    // Create template directories
    fs::create_dir_all(&template_root.join("python/agents")).unwrap();
    
    // Create a simple template for testing
    let template_content = "# Template for {{agent_id}} agent\nclass {{agent_id}}Agent:\n    model_path = \"{{model_path}}\"\n    # Configuration\n\n# Values: model_path={{model_path}}";
    fs::write(template_root.join("python/agents/ml_model.py.tmpl"), template_content).unwrap();
    
    // Initialize the Python generator
    let mut python_generator = PythonGenerator::new(&template_root, &output_dir).unwrap();
    
    // Create a test agent
    let agent = Agent {
        id: Some("TestMLModel".to_string()),
        agent_type: AgentType::MLModel,
        config: vec![
            Argument::Named("model_path".to_string(), Value::String("/path/to/model.pkl".to_string())),
            Argument::Named("batch_size".to_string(), Value::Number(32.0)),
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
    
    // Generate code for the ML Model agent
    let result = python_generator.generate_ml_model_agent(&agent, &workflow);
    assert!(result.is_ok(), "Failed to generate ML Model agent code");
    
    // Check the generated file
    let output_file = result.unwrap();
    assert!(output_file.exists(), "Output file was not created");
    
    // Check the content of the file
    let file_content = fs::read_to_string(&output_file).unwrap();
    assert!(file_content.contains("TestMLModel"), "Agent ID not found in generated code");
    assert!(file_content.contains("/path/to/model.pkl"), "Model path parameter not found in generated code");
}

#[test]
fn test_bayesian_network_agent_generation() {
    // Create a temporary directory for templates and output
    let temp_dir = tempfile::tempdir().unwrap();
    let template_root = temp_dir.path().join("templates");
    let output_dir = temp_dir.path().join("output");
    
    // Create template directories
    fs::create_dir_all(&template_root.join("python/agents")).unwrap();
    
    // Create a simple template for testing
    let template_content = "# Template for {{agent_id}} agent\nclass {{agent_id}}Agent:\n    network_path = \"{{network_path}}\"\n    # Configuration\n\n# Values: network_path={{network_path}}";
    fs::write(template_root.join("python/agents/bayesian_network.py.tmpl"), template_content).unwrap();
    
    // Initialize the Python generator
    let mut python_generator = PythonGenerator::new(&template_root, &output_dir).unwrap();
    
    // Create a test agent
    let agent = Agent {
        id: Some("TestBayesianNetwork".to_string()),
        agent_type: AgentType::BayesianNetwork,
        config: vec![
            Argument::Named("network_path".to_string(), Value::String("/path/to/network.bn".to_string())),
            Argument::Named("inference_method".to_string(), Value::String("variable_elimination".to_string())),
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
    
    // Generate code for the Bayesian Network agent
    let result = python_generator.generate_bayesian_network_agent(&agent, &workflow);
    assert!(result.is_ok(), "Failed to generate Bayesian Network agent code");
    
    // Check the generated file
    let output_file = result.unwrap();
    assert!(output_file.exists(), "Output file was not created");
    
    // Check the content of the file
    let file_content = fs::read_to_string(&output_file).unwrap();
    assert!(file_content.contains("TestBayesianNetwork"), "Agent ID not found in generated code");
    assert!(file_content.contains("/path/to/network.bn"), "Network path parameter not found in generated code");
}

#[test]
fn test_decision_matrix_agent_generation() {
    // Create a temporary directory for templates and output
    let temp_dir = tempfile::tempdir().unwrap();
    let template_root = temp_dir.path().join("templates");
    let output_dir = temp_dir.path().join("output");
    
    // Create template directories
    fs::create_dir_all(&template_root.join("python/agents")).unwrap();
    
    // Create a simple template for testing
    let template_content = "# Template for {{agent_id}} agent\nclass {{agent_id}}Agent:\n    matrix_definition = \"{{matrix_definition}}\"\n    # Configuration\n\n# Values: matrix_definition={{matrix_definition}}";
    fs::write(template_root.join("python/agents/decision_matrix.py.tmpl"), template_content).unwrap();
    
    // Initialize the Python generator
    let mut python_generator = PythonGenerator::new(&template_root, &output_dir).unwrap();
    
    // Create a test agent
    let agent = Agent {
        id: Some("TestDecisionMatrix".to_string()),
        agent_type: AgentType::DecisionMatrix,
        config: vec![
            Argument::Named("matrix_definition".to_string(), Value::String("/path/to/matrix.dmx".to_string())),
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
    
    // Generate code for the Decision Matrix agent
    let result = python_generator.generate_decision_matrix_agent(&agent, &workflow);
    assert!(result.is_ok(), "Failed to generate Decision Matrix agent code");
    
    // Check the generated file
    let output_file = result.unwrap();
    assert!(output_file.exists(), "Output file was not created");
    
    // Check the content of the file
    let file_content = fs::read_to_string(&output_file).unwrap();
    assert!(file_content.contains("TestDecisionMatrix"), "Agent ID not found in generated code");
    assert!(file_content.contains("/path/to/matrix.dmx"), "Matrix definition parameter not found in generated code");
}

use anyhow::Result;
use kumeo_compiler::{
    ast::{Agent, AgentType},
    codegen::agent::generate_agent,
};
use std::path::Path;
use tempfile::tempdir;
use tera::Tera;

#[test]
fn test_generate_agent() -> Result<()> {
    // Create a temporary directory for the test
    let _temp_dir = tempdir()?;
    let output_dir = tempdir()?;
    
    // Create a test agent with LLM type
    let agent = Agent {
        id: Some("test-agent".to_string()),
        agent_type: AgentType::LLM,
        config: vec![],
    };
    
    // Create templates directory for LLM agent
    let templates_dir = std::env::current_dir()?.join("compiler/templates/agents/llm");
    std::fs::create_dir_all(&templates_dir)?;
    
    // Create a simple Dockerfile template
    std::fs::write(
        templates_dir.join("Dockerfile.tera"),
        "FROM python:3.9-slim\nWORKDIR /app\nCOPY . .\nRUN pip install -r requirements.txt"
    )?;
    
    // Create a simple README template
    std::fs::write(
        templates_dir.join("README.md.tera"),
        "# {{ agent_id }}\n\nThis is an LLM agent."
    )?;
    
    // Create a simple k8s deployment template
    std::fs::create_dir_all(templates_dir.join("kubernetes"))?;
    std::fs::write(
        templates_dir.join("kubernetes/deployment.yaml.tera"),
        "apiVersion: apps/v1\nkind: Deployment\nmetadata:\n  name: {{ agent_id }}\nspec:\n  replicas: 1\n  selector:\n    matchLabels:\n      app: {{ agent_id }}\n  template:\n    metadata:\n      labels:\n        app: {{ agent_id }}\n    spec:\n      containers:\n      - name: {{ agent_id }}\n        image: {{ agent_id }}\n        ports:\n        - containerPort: 8080"
    )?;
    
    // Initialize Tera
    let tera = Tera::default();
    
    // Generate agent files
    println!("Output directory: {}", output_dir.path().display());
    generate_agent(&agent, output_dir.path(), &tera)?;
    
    // Verify output directory structure
    let agent_dir = output_dir.path().join("agents/test-agent");
    println!("Agent directory: {}", agent_dir.display());
    println!("Agent directory exists: {}", agent_dir.exists());
    
    if agent_dir.exists() {
        println!("Agent directory contents:");
        for entry in std::fs::read_dir(&agent_dir)? {
            let entry = entry?;
            println!("  - {} (dir: {})", entry.file_name().to_string_lossy(), entry.file_type()?.is_dir());
        }
    }
    
    assert!(agent_dir.exists(), "Agent directory was not created");
    assert!(agent_dir.is_dir(), "Agent path is not a directory");
    
    // Verify Dockerfile was generated
    let dockerfile = agent_dir.join("Dockerfile");
    println!("Dockerfile path: {}", dockerfile.display());
    println!("Dockerfile exists: {}", dockerfile.exists());
    assert!(dockerfile.exists(), "Dockerfile was not generated");
    
    // Verify Kubernetes manifests were generated
    let k8s_dir = agent_dir.join("kubernetes");
    println!("Kubernetes dir: {}", k8s_dir.display());
    println!("Kubernetes dir exists: {}", k8s_dir.exists());
    if k8s_dir.exists() {
        println!("Kubernetes dir contents:");
        for entry in std::fs::read_dir(&k8s_dir)? {
            let entry = entry?;
            println!("  - {}", entry.file_name().to_string_lossy());
        }
    }
    assert!(k8s_dir.exists(), "Kubernetes directory was not generated");
    assert!(k8s_dir.is_dir(), "Kubernetes path is not a directory");
    
    // Verify README was generated
    let readme = agent_dir.join("README.md");
    println!("README path: {}", readme.display());
    println!("README exists: {}", readme.exists());
    assert!(readme.exists(), "README was not generated");
    
    Ok(())
}

#[test]
fn test_generate_agent_with_config() -> Result<()> {
    // Create a temporary directory for the test
    let _temp_dir = tempdir()?;
    let output_dir = tempdir()?;
    
    // Create a test agent with config
    let agent = Agent {
        id: Some("config-agent".to_string()),
        agent_type: AgentType::LLM,
        config: vec![],
    };
    
    // Initialize Tera
    let tera = Tera::default();
    
    // Generate agent files
    generate_agent(&agent, output_dir.path(), &tera)?;
    
    // Verify output directory structure
    let agent_dir = output_dir.path().join("agents/config-agent");
    assert!(agent_dir.exists());
    assert!(agent_dir.is_dir());
    
    Ok(())
}

#[test]
fn test_generate_agent_without_id() {
    // Create a test agent without an ID
    let agent = Agent {
        id: None,
        agent_type: AgentType::LLM,
        config: vec![],
    };
    
    // Initialize Tera
    let tera = Tera::default();
    
    // This should fail because the agent doesn't have an ID
    let result = generate_agent(&agent, Path::new("."), &tera);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Agent must have an ID"
    );
}

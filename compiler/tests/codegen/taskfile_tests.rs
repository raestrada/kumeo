use anyhow::Result;
use kumeo_compiler::{
    ast::{Workflow, Agent, AgentType},
    codegen::taskfile::generate_taskfiles,
};
use std::path::Path;
use tempfile::tempdir;
use tera::Tera;

#[test]
fn test_generate_taskfiles() -> Result<()> {
    // Create a temporary directory for the test
    let output_dir = tempdir()?;
    
    // Create a test workflow with agents
    let workflow = Workflow {
        name: "test-workflow".to_string(),
        source: None,
        target: None,
        context: None,
        preprocessors: None,
        agents: vec![
            Agent {
                id: Some("rust-agent".to_string()),
                agent_type: AgentType::LLM,
                config: vec![],
            },
            Agent {
                id: Some("python-agent".to_string()),
                agent_type: AgentType::MLModel,
                config: vec![],
            },
        ],
        monitor: None,
        deployment: None,
    };
    
    // Initialize Tera
    let tera = Tera::default();
    
    // Generate taskfiles
    generate_taskfiles(&workflow, output_dir.path(), &tera)?;
    
    // Verify Taskfile was generated
    let taskfile = output_dir.path().join("Taskfile.yaml");
    assert!(taskfile.exists());
    
    // Verify task files for each agent type were generated
    let rust_tasks = output_dir.path().join("tasks/rust/Taskfile.yaml");
    let python_tasks = output_dir.path().join("tasks/python/Taskfile.yaml");
    
    assert!(rust_tasks.exists());
    assert!(python_tasks.exists());
    
    Ok(())
}

#[test]
fn test_generate_taskfiles_with_custom_templates() -> Result<()> {
    // Create a temporary directory for the test
    let temp_dir = tempdir()?;
    let output_dir = tempdir()?;
    
    // Create a test workflow with an agent
    let workflow = Workflow {
        name: "custom-templates".to_string(),
        source: None,
        target: None,
        context: None,
        preprocessors: None,
        agents: vec![Agent {
            id: Some("test-agent".to_string()),
            agent_type: AgentType::LLM,
            config: vec![],
        }],
        monitor: None,
        deployment: None,
    };
    
    // Create custom task templates
    let template_dir = temp_dir.path().join("templates/tasks");
    std::fs::create_dir_all(template_dir.join("rust"))?;
    std::fs::create_dir_all(template_dir.join("python"))?;
    
    // Create a custom Rust task template
    std::fs::write(
        template_dir.join("rust/Taskfile.yaml.tera"),
        "version: '3'\n\nvars:\n  AGENT_NAME: {{ agent_id }}\n\ntasks:\n  build:\n    desc: Build the Rust agent\n    cmds:\n      - cargo build --release\n\n  test:\n    desc: Test the Rust agent\n    cmds:\n      - cargo test\n",
    )?;
    
    // Create a custom Python task template
    std::fs::write(
        template_dir.join("python/Taskfile.yaml.tera"),
        "version: '3'\n\nvars:\n  AGENT_NAME: {{ agent_id }}\n\ntasks:\n  install:\n    desc: Install Python dependencies\n    cmds:\n      - pip install -r requirements.txt\n\n  test:\n    desc: Run Python tests\n    cmds:\n      - python -m pytest\n",
    )?;
    
    // Initialize Tera with custom templates
    let tera = Tera::new(&format!("{}/**/*.tera", template_dir.display()))?;
    
    // Generate taskfiles with custom templates
    generate_taskfiles(&workflow, output_dir.path(), &tera)?;
    
    // Verify custom task files were used
    let rust_tasks = output_dir.path().join("tasks/rust/Taskfile.yaml");
    let python_tasks = output_dir.path().join("tasks/python/Taskfile.yaml");
    
    assert!(rust_tasks.exists());
    assert!(python_tasks.exists());
    
    // Verify custom content was rendered
    let rust_content = std::fs::read_to_string(rust_tasks)?;
    assert!(rust_content.contains("Build the Rust agent"));
    
    let python_content = std::fs::read_to_string(python_tasks)?;
    assert!(python_content.contains("Install Python dependencies"));
    
    Ok(())
}

#[test]
fn test_generate_taskfiles_without_agents() -> Result<()> {
    // Create a temporary directory for the test
    let output_dir = tempdir()?;
    
    // Create a test workflow without agents
    let workflow = Workflow {
        name: "no-agents".to_string(),
        source: None,
        target: None,
        context: None,
        preprocessors: None,
        agents: vec![],
        monitor: None,
        deployment: None,
    };
    
    // Initialize Tera
    let tera = Tera::default();
    
    // Generate taskfiles
    let result = generate_taskfiles(&workflow, output_dir.path(), &tera);
    
    // Should succeed even with no agents
    assert!(result.is_ok());
    
    // Taskfile should still be created
    let taskfile = output_dir.path().join("Taskfile.yaml");
    assert!(taskfile.exists());
    
    // But no agent-specific task files should be created
    let tasks_dir = output_dir.path().join("tasks");
    assert!(!tasks_dir.exists());
    
    Ok(())
}

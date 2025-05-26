use anyhow::Result;
use kumeo_compiler::{
    ast::{Workflow, Agent, AgentType},
    codegen::kubernetes::generate_kubernetes_config,
};
use std::path::Path;
use tempfile::tempdir;
use tera::Tera;

#[test]
fn test_generate_kubernetes_config() -> Result<()> {
    // Create a temporary directory for the test
    let temp_dir = tempdir()?;
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
                id: Some("test-agent-1".to_string()),
                agent_type: AgentType::LLM,
                config: vec![],
            },
            Agent {
                id: Some("test-agent-2".to_string()),
                agent_type: AgentType::MLModel,
                config: vec![],
            },
        ],
        monitor: None,
        deployment: None,
    };
    
    // Initialize Tera
    let tera = Tera::default();
    
    // Generate Kubernetes configuration
    generate_kubernetes_config(&workflow, output_dir.path(), &tera)?;
    
    // Verify output directory structure
    let kubernetes_dir = output_dir.path().join("kubernetes");
    assert!(kubernetes_dir.exists());
    assert!(kubernetes_dir.is_dir());
    
    // Verify Helm chart was generated
    let helm_dir = kubernetes_dir.join("helm").join("test-workflow");
    assert!(helm_dir.exists());
    assert!(helm_dir.is_dir());
    
    // Verify values.yaml was generated
    let values_file = helm_dir.join("values.yaml");
    assert!(values_file.exists());
    
    Ok(())
}

#[test]
fn test_generate_kubernetes_with_custom_templates() -> Result<()> {
    // Create a temporary directory for the test
    let temp_dir = tempdir()?;
    let output_dir = tempdir()?;
    
    // Create a test workflow
    let workflow = Workflow {
        name: "custom-templates".to_string(),
        source: None,
        target: None,
        context: None,
        preprocessors: None,
        agents: vec![],
        monitor: None,
        deployment: None,
    };
    
    // Create custom templates
    let template_dir = temp_dir.path().join("templates/kubernetes");
    std::fs::create_dir_all(template_dir.join("helm/templates"))?;
    
    // Create a custom Helm template
    std::fs::write(
        template_dir.join("helm/templates/configmap.yaml.tera"),
        "apiVersion: v1\nkind: ConfigMap\nmetadata:\n  name: {{ .Values.workflow.name }}-config\ndata:\n  config.yaml: |\n    workflow:\n      name: {{ .Values.workflow.name }}",
    )?;
    
    // Initialize Tera with custom templates
    let tera = Tera::new(&format!("{}/**/*.tera", template_dir.display()))?;
    
    // Generate Kubernetes configuration
    generate_kubernetes_config(&workflow, output_dir.path(), &tera)?;
    
    // Verify custom template was processed
    let config_map = output_dir
        .path()
        .join("kubernetes/helm/custom-templates/templates/configmap.yaml");
    assert!(config_map.exists());
    
    // Verify the content was rendered correctly
    let content = std::fs::read_to_string(config_map)?;
    assert!(content.contains("name: custom-templates-config"));
    
    Ok(())
}

#[test]
fn test_count_agent_types() {
    use kumeo_compiler::codegen::kubernetes::count_agent_types;
    
    // Create a test workflow with agents
    let workflow = Workflow {
        name: "test-count".to_string(),
        source: None,
        target: None,
        context: None,
        preprocessors: None,
        agents: vec![
            Agent {
                id: Some("agent1".to_string()),
                agent_type: AgentType::LLM,
                config: vec![],
            },
            Agent {
                id: Some("agent2".to_string()),
                agent_type: AgentType::MLModel,
                config: vec![],
            },
            Agent {
                id: Some("agent3".to_string()),
                agent_type: AgentType::LLM,
                config: vec![],
            },
        ],
        monitor: None,
        deployment: None,
    };
    
    let counts = count_agent_types(&workflow);
    
    assert_eq!(counts.get("llm"), Some(&2));
    assert_eq!(counts.get("mlmodel"), Some(&1));
    assert_eq!(counts.get("nonexistent"), None);
}

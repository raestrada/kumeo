//! Agent code generation

use anyhow::Result;
use std::path::{Path, PathBuf};
use tera::Tera;

use crate::ast::{Agent, AgentType};
use super::template_processor::{process_template_dir, create_base_context};
use anyhow::Context;

/// Generate agent-specific files based on agent type
pub fn generate_agent(agent: &Agent, output_dir: &Path, tera: &Tera) -> Result<()> {
    // Get agent ID or return error if missing
    let agent_id = agent.id.as_ref().ok_or_else(|| 
        anyhow::anyhow!("Agent must have an ID")
    )?;

    // Determine agent type and template directory
    let (agent_type, template_dir) = match agent.agent_type {
        AgentType::LLM => ("llm", "llm"),
        AgentType::MLModel => ("mlmodel", "mlmodel"),
        AgentType::DataProcessor => ("dataprocessor", "dataprocessor"),
        AgentType::Router => ("router", "router"),
        AgentType::DecisionMatrix => ("decisionmatrix", "decisionmatrix"),
        AgentType::HumanReview => ("humanreview", "humanreview"),
    };

    // Create agent context
    let mut context = create_base_context(agent_id);
    context.insert("agent", agent);
    context.insert("agent_type", &agent.agent_type);
    context.insert("agent_id", agent_id);
    
    // Use agent ID as the name
    context.insert("agent_name", agent_id);

    // Create agent directory based on type and name
    let agent_dir = output_dir.join(format!("agents/{}", agent_id));
    std::fs::create_dir_all(&agent_dir)
        .with_context(|| format!("Failed to create agent directory: {}", agent_dir.display()))?;

    // Process template directory
    let template_path = PathBuf::from("templates/agents").join(template_dir);
    if template_path.exists() {
        process_template_dir(&template_path, &agent_dir, &context, tera, &[])
            .with_context(|| format!("Failed to process template for agent: {}", agent_id))?;
    } else {
        // Fallback to default agent template if specific template doesn't exist
        let default_template_path = PathBuf::from("templates/agents/default");
        if default_template_path.exists() {
            process_template_dir(&default_template_path, &agent_dir, &context, tera, &[])
                .with_context(|| format!("Failed to process default template for agent: {}", agent_id))?;
        }
    }

    // Generate Dockerfile
    generate_dockerfile(agent, &agent_dir, &context, tera)?;
    
    // Generate Kubernetes manifests
    generate_kubernetes_manifests(agent, &agent_dir, &context, tera).ok(); // Ignore errors for now
    
    // Generate README for the agent
    generate_readme(agent, &agent_dir, &context, tera)?;

    Ok(())
}

/// Generate Dockerfile for the agent
fn generate_dockerfile(
    agent: &Agent,
    output_dir: &Path,
    context: &tera::Context,
    tera: &Tera,
) -> Result<()> {
    let dockerfile_template = match agent.agent_type {
        AgentType::MLModel => "agents/mlmodel/Dockerfile.tera",
        _ => "agents/default/Dockerfile.tera",
    };

    let output_path = output_dir.join("Dockerfile");
    
    // Only generate if template exists
    if let Some(template) = tera.get_template_names().find(|&name| name == dockerfile_template) {
        if let Ok(rendered) = tera.render(template, context) {
            std::fs::write(&output_path, rendered)
                .with_context(|| format!("Failed to write Dockerfile: {}", output_path.display()))?;
        }
    }

    Ok(())
}

/// Generate Kubernetes manifests for an agent
fn generate_kubernetes_manifests(
    _agent: &Agent,
    output_dir: &Path,
    context: &tera::Context,
    tera: &Tera,
) -> Result<()> {
    let k8s_dir = output_dir.join("kubernetes");
    std::fs::create_dir_all(&k8s_dir).ok();
    
    let template_dir = PathBuf::from("compiler/templates/kubernetes/agent");
    if template_dir.exists() {
        process_template_dir(&template_dir, &k8s_dir, context, tera, &[]).ok();
    }
    
    Ok(())
}

/// Generate a README file for the agent
fn generate_readme(
    _agent: &Agent,
    output_dir: &Path,
    context: &tera::Context,
    tera: &Tera,
) -> Result<()> {
    let readme_path = output_dir.join("README.md");
    
    // Only generate if template exists
    if let Some(template) = tera.get_template_names().find(|&name| name == "agents/README.md.tera") {
        if let Ok(rendered) = tera.render(template, context) {
            std::fs::write(&readme_path, rendered).ok();
        }
    }
    
    Ok(())
}

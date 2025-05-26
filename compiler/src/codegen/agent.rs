//! Agent code generation

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tera::Tera;

use crate::ast::{Agent, AgentType};
use super::template_processor::{process_template_dir, create_base_context};

/// Generate agent-specific files based on agent type
pub fn generate_agent(agent: &Agent, output_dir: &Path, tera: &Tera) -> Result<()> {
    // Determine agent type and template directory
    let (agent_type, template_dir) = match agent.agent_type {
        AgentType::LLM => ("rust", "rust"),
        AgentType::MLModel => ("python", "python"),
        AgentType::DataProcessor => ("rust", "rust"),
        AgentType::Router => ("rust", "rust"),
        AgentType::DecisionMatrix => ("rust", "rust"),
        AgentType::HumanReview => ("rust", "rust"),
    };

    // Get agent ID or return error if missing
    let agent_id = agent.id.as_ref().ok_or_else(|| 
        anyhow::anyhow!("Agent must have an ID")
    )?;

    // Create agent context
    let mut context = create_base_context(agent_id);
    context.insert("agent", agent);
    context.insert("agent_type", &agent.agent_type);
    context.insert("agent_id", agent_id);

    // Create agent directory based on type and name
    let agent_dir = output_dir.join(format!("{}s/{}-{}", agent_type, agent_type, agent_id));
    std::fs::create_dir_all(&agent_dir)
        .with_context(|| format!("Failed to create agent directory: {}", agent_dir.display()))?;

    // Process template directory
    let template_path = PathBuf::from("compiler/templates").join(template_dir);
    process_template_dir(&template_path, &agent_dir, &context, tera)
        .with_context(|| format!("Failed to process template for agent: {}", agent_id))?;

    // Generate Dockerfile if it's a template
    generate_dockerfile(agent, &agent_dir, &context, tera)?;

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
        AgentType::LLM => "rust/rust.Dockerfile.tera",
        AgentType::MLModel => "python/python.Dockerfile.tera",
        AgentType::DataProcessor => return Ok(()), // No se necesita Dockerfile especial para DataProcessor
        AgentType::Router => "rust/rust.Dockerfile.tera",
        AgentType::DecisionMatrix => "rust/rust.Dockerfile.tera",
        AgentType::HumanReview => "rust/rust.Dockerfile.tera",
    };

    let output_path = output_dir.join("Dockerfile");
    let rendered = tera
        .render(dockerfile_template, context)
        .with_context(|| format!("Failed to render Dockerfile for agent: {:?}", agent.id))?;

    std::fs::write(&output_path, rendered)
        .with_context(|| format!("Failed to write Dockerfile: {}", output_path.display()))?;

    Ok(())
}

//! Taskfile generation

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tera::Tera;

use crate::ast::{Workflow, AgentType};
use super::template_processor::{process_template_dir, create_base_context};

/// Generate Taskfile and related task configurations
pub fn generate_taskfiles(
    workflow: &Workflow,
    output_dir: &Path,
    tera: &Tera,
) -> Result<()> {
    let tasks_dir = output_dir.join("tasks");
    std::fs::create_dir_all(&tasks_dir)
        .with_context(|| format!("Failed to create tasks directory: {}", tasks_dir.display()))?;

    // Create context with workflow information
    let mut context = create_base_context(&workflow.name);
    
    // Categorize agents by type
    let rust_agents: Vec<_> = workflow.agents
        .iter()
        .filter(|a| matches!(
            a.agent_type,
            AgentType::LLM | 
            AgentType::DataProcessor | 
            AgentType::Router | 
            AgentType::DecisionMatrix | 
            AgentType::HumanReview
        ))
        .collect();
    
    let python_agents: Vec<_> = workflow.agents
        .iter()
        .filter(|a| matches!(a.agent_type, AgentType::MLModel))
        .collect();
    
    context.insert("rust_agents", &rust_agents);
    context.insert("python_agents", &python_agents);

    // Process task templates
    let template_dir = PathBuf::from("compiler/templates/tasks");
    if template_dir.exists() {
        // Generate main Taskfile
        let taskfile_template = template_dir.join("Taskfile.yml.tera");
        if taskfile_template.exists() {
            let rendered = tera
                .render(taskfile_template.to_str().unwrap(), &context)
                .context("Failed to render Taskfile")?;
            
            std::fs::write(output_dir.join("Taskfile.yml"), rendered)
                .context("Failed to write Taskfile")?;
        }

        // Process other task templates
        process_template_dir(&template_dir, &tasks_dir, &context, tera)
            .context("Failed to process task templates")?;
    }

    Ok(())
}

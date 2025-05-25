//! Code generation module for Kumeo
//! 
//! This module handles the generation of project files from templates
//! based on the parsed DSL.

mod agent;
mod kubernetes;
mod taskfile;
mod template_processor;

use anyhow::Result;
use std::path::Path;
use tera::Tera;

use crate::ast::Workflow;

/// Generate all project files from templates
pub fn generate_workflow(workflow: &Workflow, output_dir: &Path) -> Result<()> {
    // Initialize template engine
    let mut tera = Tera::new("compiler/templates/**/*.tera")?;
    tera.autoescape_on(vec![".rs", ".toml", ".yaml", ".yml", ".py"]);

    // Generate Kubernetes configuration
    kubernetes::generate_kubernetes_config(workflow, output_dir, &tera)?;

    // Generate Taskfiles
    taskfile::generate_taskfiles(workflow, output_dir, &tera)?;

    // Generate agent-specific files
    for agent in &workflow.agents {
        agent::generate_agent(agent, output_dir, &tera)?;
    }

    Ok(())
}

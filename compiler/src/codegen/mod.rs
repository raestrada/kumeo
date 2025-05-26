//! Code generation module for Kumeo
//! 
//! This module handles the generation of project files from templates
//! based on the parsed DSL.

use anyhow::Context;

pub mod agent;
pub mod kubernetes;
pub mod taskfile;
pub mod template_processor;

use anyhow::Result;
use std::path::Path;
use tera::Tera;

use crate::ast::Workflow;

/// Generate all project files from templates
pub fn generate_workflow(workflow: &Workflow, output_dir: &Path) -> Result<()> {
    // Initialize template engine
    let mut tera = Tera::new("compiler/templates/**/*.tera")?;
    tera.autoescape_on(vec![".rs", ".toml", ".yaml", ".yml", ".py"]);

    // Create output directory if it doesn't exist
    std::fs::create_dir_all(output_dir)
        .with_context(|| format!("Failed to create output directory: {}", output_dir.display()))?;

    // Generate Kubernetes configuration
    kubernetes::generate_kubernetes_config(workflow, output_dir, &tera)?;

    // Generate Taskfiles
    taskfile::generate_taskfiles(workflow, output_dir, &tera)?;

    // Generate agent-specific files
    for agent in &workflow.agents {
        agent::generate_agent(agent, output_dir, &tera)?;
    }

    // Generate workflow-level files
    generate_workflow_files(workflow, output_dir, &tera)?;

    Ok(())
}

/// Generate workflow-level files
fn generate_workflow_files(workflow: &Workflow, output_dir: &Path, tera: &Tera) -> Result<()> {
    let mut context = template_processor::create_base_context(&workflow.name);
    context.insert("workflow", workflow);
    
    // Generate README.md for the workflow
    let readme_path = output_dir.join("README.md");
    if let Ok(rendered) = tera.render("workflow/README.md.tera", &context) {
        std::fs::write(readme_path, rendered)?;
    }
    
    // Generate .gitignore if it doesn't exist
    let gitignore_path = output_dir.join(".gitignore");
    if !gitignore_path.exists() {
        if let Ok(rendered) = tera.render("workflow/gitignore.tera", &context) {
            std::fs::write(gitignore_path, rendered)?;
        }
    }
    
    Ok(())
}

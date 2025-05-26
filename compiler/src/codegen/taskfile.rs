//! Taskfile generation

use anyhow::Result;
use std::path::Path;
use tera::Tera;
use std::collections::{HashMap, HashSet};

use crate::ast::{Workflow, Agent, AgentType};
use super::template_processor::create_base_context;

/// Generate Taskfile and related task configurations
pub fn generate_taskfiles(
    workflow: &Workflow,
    output_dir: &Path,
    tera: &Tera,
) -> Result<()> {
    let tasks_dir = output_dir.join("tasks");
    std::fs::create_dir_all(&tasks_dir)?;
    
    // Group agents by language and type
    let mut agents_by_lang: HashMap<String, Vec<&Agent>> = HashMap::new();
    let mut agent_types = HashSet::new();
    
    for agent in &workflow.agents {
        let lang = match agent.agent_type {
            AgentType::LLM | AgentType::MLModel => "python",
            AgentType::DataProcessor | AgentType::Router => "rust",
            _ => "other",
        }.to_string();
        
        agents_by_lang.entry(lang).or_default().push(agent);
        agent_types.insert(agent.agent_type.to_string());
    }
    
    // Create base context
    let mut context = create_base_context(&workflow.name);
    context.insert("workflow", workflow);
    context.insert("agent_types", &agent_types);
    
    // Generate main Taskfile
    let taskfile_path = output_dir.join("Taskfile.yml");
    if let Some(template) = tera.get_template_names().find(|&name| name == "Taskfile.yml.tera") {
        if let Ok(rendered) = tera.render(template, &context) {
            std::fs::write(&taskfile_path, rendered).ok();
        }
    }
    
    // Generate language-specific task files
    for (lang, agents) in agents_by_lang {
        let lang_dir = tasks_dir.join(&lang);
        if std::fs::create_dir_all(&lang_dir).is_err() {
            continue;
        }
        
        let mut lang_context = context.clone();
        lang_context.insert("agents", &agents);
        lang_context.insert("language", &lang);
        
        let tasks_file = lang_dir.join("tasks.yml");
        if let Some(template) = tera.get_template_names().find(|&name| name == "tasks/tasks.yml.tera") {
            if let Ok(rendered) = tera.render(template, &lang_context) {
                std::fs::write(&tasks_file, rendered).ok();
            }
        }
    }

    Ok(())
}

/// Generate language-specific task files
fn generate_language_tasks(
    workflow: &Workflow,
    tasks_dir: &Path,
    context: &tera::Context,
    tera: &Tera,
) -> Result<()> {
    // Generate Rust tasks if there are Rust agents
    if workflow.agents.iter().any(|a| !matches!(a.agent_type, AgentType::MLModel)) {
        let rust_tasks_dir = tasks_dir.join("rust");
        if std::fs::create_dir_all(&rust_tasks_dir).is_ok() {
            if let Ok(rendered) = tera.render("tasks/rust/tasks.yml.tera", context) {
                std::fs::write(rust_tasks_dir.join("tasks.yml"), rendered).ok();
            }
        }
    }
    
    // Generate Python tasks if there are Python agents
    if workflow.agents.iter().any(|a| matches!(a.agent_type, AgentType::MLModel)) {
        let python_tasks_dir = tasks_dir.join("python");
        if std::fs::create_dir_all(&python_tasks_dir).is_ok() {
            if let Ok(rendered) = tera.render("tasks/python/tasks.yml.tera", context) {
                std::fs::write(python_tasks_dir.join("tasks.yml"), rendered).ok();
            }
        }
    }
    
    Ok(())
}

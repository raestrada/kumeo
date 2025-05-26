//! Kubernetes configuration generation

use anyhow::Result;
use std::path::{Path, PathBuf};
use tera::Tera;
use std::collections::HashMap;

use crate::ast::{Workflow, AgentType};
use super::template_processor::{process_template_dir, create_base_context};
use anyhow::Context;

/// Generate Kubernetes configuration files
pub fn generate_kubernetes_config(
    workflow: &Workflow,
    output_dir: &Path,
    tera: &Tera,
) -> Result<()> {
    let kubernetes_dir = output_dir.join("kubernetes");
    std::fs::create_dir_all(&kubernetes_dir)
        .with_context(|| format!("Failed to create kubernetes directory: {}", kubernetes_dir.display()))?;

    // Create context with workflow information
    let mut context = create_base_context(&workflow.name);
    context.insert("workflow", workflow);
    context.insert("namespace", "kumeo");
    context.insert("registry", "");
    context.insert("tag", "latest");
    
    // Add agent type counts to context
    let agent_type_counts = count_agent_types(&workflow);
    context.insert("agent_type_counts", &agent_type_counts);

    // Process kubernetes templates
    let template_dir = PathBuf::from("compiler/templates/kubernetes");
    if template_dir.exists() {
        // Skip agent-specific templates as they are handled in agent.rs
        process_template_dir(&template_dir, &kubernetes_dir, &context, tera, &["agent"]).ok();
    }

    // Generate Helm chart if templates exist
    let helm_dir = template_dir.join("helm");
    if helm_dir.exists() {
        let output_helm = kubernetes_dir.join("helm").join(&workflow.name);
        std::fs::create_dir_all(&output_helm)?;
        
        // Process Helm templates
        process_template_dir(&helm_dir, &output_helm, &context, tera, &[]).ok();
            
        // Generate values.yaml if it doesn't exist
        let values_path = output_helm.join("values.yaml");
        if !values_path.exists() {
            let mut values_context = tera::Context::new();
            values_context.insert("workflow", workflow);
            values_context.insert("agent_type_counts", &agent_type_counts);
            
            if let Ok(rendered) = tera.render("kubernetes/helm/values.yaml.tera", &values_context) {
                std::fs::write(values_path, rendered).ok();
            }
        }
    }

    Ok(())
}

/// Count the number of agents of each type
pub fn count_agent_types(workflow: &Workflow) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    
    for agent in &workflow.agents {
        let type_name = match agent.agent_type {
            AgentType::LLM => "llm",
            AgentType::MLModel => "mlmodel",
            AgentType::DataProcessor => "dataprocessor",
            AgentType::Router => "router",
            AgentType::DecisionMatrix => "decisionmatrix",
            AgentType::HumanReview => "humanreview",
        };
        
        *counts.entry(type_name.to_string()).or_insert(0) += 1;
    }
    
    counts
}

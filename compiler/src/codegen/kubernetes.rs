//! Kubernetes configuration generation

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tera::Tera;

use crate::ast::Workflow;
use super::template_processor::{process_template_dir, create_base_context};

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
    context.insert("agents", &workflow.agents);
    context.insert("namespace", "kumeo");
    context.insert("registry", "");
    context.insert("tag", "latest");

    // Process kubernetes templates
    let template_dir = PathBuf::from("compiler/templates/kubernetes");
    if template_dir.exists() {
        process_template_dir(&template_dir, &kubernetes_dir, &context, tera)
            .context("Failed to process kubernetes templates")?;
    }

    // Generate Helm chart if templates exist
    let helm_dir = template_dir.join("helm");
    if helm_dir.exists() {
        let output_helm = kubernetes_dir.join("helm").join(&workflow.name);
        process_template_dir(&helm_dir, &output_helm, &context, tera)
            .context("Failed to process helm templates")?;
    }

    Ok(())
}

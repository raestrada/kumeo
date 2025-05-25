//! Template processing utilities for code generation

use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use tera::Tera;

/// Process a directory of templates and render them to the output directory
pub fn process_template_dir(
    template_dir: &Path,
    output_dir: &Path,
    context: &tera::Context,
    tera: &Tera,
) -> Result<()> {
    // Create output directory if it doesn't exist
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)
            .with_context(|| format!("Failed to create directory: {}", output_dir.display()))?;
    }

    // Process each entry in the template directory
    for entry in fs::read_dir(template_dir)? {
        let entry = entry?;
        let entry_path = entry.path();
        let file_name = entry_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| anyhow::anyhow!("Invalid file name"))?;

        let output_path = output_dir.join(file_name);

        if entry_path.is_dir() {
            // Recursively process subdirectories
            process_template_dir(&entry_path, &output_path, context, tera)?;
        } else {
            // Process template files
            let template_path = entry_path.to_str().ok_or_else(|| {
                anyhow::anyhow!("Invalid template path: {}", entry_path.display())
            })?;

            // Remove .tera extension from output filename
            let output_path = if file_name.ends_with(".tera") {
                output_path.with_extension("")
            } else {
                output_path
            };

            // Render template
            let rendered = tera
                .render(template_path, context)
                .with_context(|| format!("Failed to render template: {}", template_path))?;

            // Write rendered content to file
            let mut file = fs::File::create(&output_path)
                .with_context(|| format!("Failed to create file: {}", output_path.display()))?;
            file.write_all(rendered.as_bytes())
                .with_context(|| format!("Failed to write file: {}", output_path.display()))?;
        }
    }

    Ok(())
}

/// Helper function to create a context with common variables
pub fn create_base_context(workflow_name: &str) -> tera::Context {
    let mut context = tera::Context::new();
    context.insert("workflow_name", workflow_name);
    context
}

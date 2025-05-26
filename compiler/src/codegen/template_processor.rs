//! Template processing utilities for code generation

use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use tera::Tera;
use std::ffi::OsStr;
use std::collections::HashSet;

/// Process a directory of templates and render them to the output directory
/// 
/// # Arguments
/// * `template_dir` - Directory containing the template files
/// * `output_dir` - Directory where the rendered files will be written
/// * `context` - Tera context with variables for template rendering
/// * `tera` - Tera template engine instance
/// * `exclude_dirs` - List of directory names to exclude from processing
pub fn process_template_dir(
    template_dir: &Path,
    output_dir: &Path,
    context: &tera::Context,
    tera: &Tera,
    exclude_dirs: &[&str],
) -> Result<()> {
    // Create output directory if it doesn't exist
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)
            .with_context(|| format!("Failed to create directory: {}", output_dir.display()))?;
    }

    // Convert exclude dirs to a set for faster lookups
    let exclude_set: HashSet<&str> = exclude_dirs.iter().cloned().collect();

    // Create a new Tera instance that knows about our template directory
    let mut tera = Tera::new(template_dir.join("**/*").to_str().unwrap())
        .with_context(|| format!("Failed to parse templates in {}", template_dir.display()))?;

    // Process each entry in the template directory
    for entry in fs::read_dir(template_dir)? {
        let entry = entry?;
        let entry_path = entry.path();
        
        // Skip if the entry is in the exclude list
        let file_name = entry_path.file_name()
            .and_then(OsStr::to_str)
            .ok_or_else(|| anyhow::anyhow!("Invalid file name"))?;
            
        if exclude_set.contains(file_name) {
            continue;
        }

        // Get the relative path from the template directory
        let relative_path = entry_path.strip_prefix(template_dir)
            .unwrap_or_else(|_| entry_path.as_path());
            
        let output_path = output_dir.join(relative_path);

        if entry_path.is_dir() {
            // Create the directory in the output path
            fs::create_dir_all(&output_path)
                .with_context(|| format!("Failed to create directory: {}", output_path.display()))?;
                
            // Recursively process subdirectories
            process_template_dir(&entry_path, &output_path, context, &tera, exclude_dirs)?;
        } else if entry_path.extension().and_then(OsStr::to_str) == Some("tera") {
            // Process template file
            let template_content = fs::read_to_string(&entry_path)
                .with_context(|| format!("Failed to read template: {}", entry_path.display()))?;
                
            // Render the template
            let rendered = match tera.render_str(&template_content, context) {
                Ok(rendered) => rendered,
                Err(e) => {
                    eprintln!("Warning: Failed to render template {}: {}", entry_path.display(), e);
                    continue;
                }
            };
            
            // Remove .tera extension from output path
            let output_path = output_path.with_extension("");
            
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
            }

            // Write rendered content to file
            let mut file = fs::File::create(&output_path)
                .with_context(|| format!("Failed to create file: {}", output_path.display()))?;
                
            file.write_all(rendered.as_bytes())
                .with_context(|| format!("Failed to write to file: {}", output_path.display()))?;
        } else {
            // Copy non-template files as-is
            fs::copy(&entry_path, &output_path)
                .with_context(|| format!("Failed to copy file from {} to {}", entry_path.display(), output_path.display()))?;
        }
    }

    Ok(())
}

/// Helper function to create a context with common variables
pub fn create_base_context(workflow_name: &str) -> tera::Context {
    let mut context = tera::Context::new();
    context.insert("workflow_name", workflow_name);
    context.insert("current_year", &chrono::Utc::now().format("%Y").to_string());
    context
}

/// Helper function to check if a file contains a specific pattern
pub fn file_contains(file_path: &Path, pattern: &str) -> Result<bool> {
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path.display()))?;
    Ok(content.contains(pattern))
}

/// Helper function to render a template string with the given context
pub fn render_template_string(template: &str, context: &tera::Context, _tera: &Tera) -> Result<String> {
    let mut tera = Tera::default();
    tera.add_raw_template("inline_template", template)
        .context("Failed to parse inline template")?;
    
    tera.render("inline_template", context)
        .context("Failed to render template string")
}

/// Helper function to create a directory if it doesn't exist
pub fn ensure_dir(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)
            .with_context(|| format!("Failed to create directory: {}", path.display()))?;
    }
    Ok(())
}

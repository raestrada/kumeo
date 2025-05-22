use std::collections::HashMap;
use std::fs;
use std::path::Path;
use kumeo_compiler::codegen::template_manager::{TemplateManager, Result, TemplateError};

#[test]
fn test_template_manager_initialization() {
    // Create a temporary directory for test templates
    let temp_dir = tempfile::tempdir().unwrap();
    let template_path = temp_dir.path();
    
    // Create a template file
    let template_content = "Hello, {{name}}!";
    let template_file_path = template_path.join("greeting.tmpl");
    fs::write(&template_file_path, template_content).unwrap();
    
    // Initialize the template manager
    let template_manager = TemplateManager::new(template_path);
    assert!(template_manager.is_ok(), "Template manager initialization failed");
}

#[test]
fn test_template_loading() {
    // Create a temporary directory for test templates
    let temp_dir = tempfile::tempdir().unwrap();
    let template_path = temp_dir.path();
    
    // Create a template file
    let template_content = "Hello, {{name}}!";
    let template_file_path = template_path.join("greeting.tmpl");
    fs::write(&template_file_path, template_content).unwrap();
    
    // Initialize the template manager and load the template
    let mut template_manager = TemplateManager::new(template_path).unwrap();
    let loaded_template = template_manager.load_template("greeting.tmpl");
    
    assert!(loaded_template.is_ok(), "Template loading failed");
    assert_eq!(loaded_template.unwrap(), template_content, "Loaded template content doesn't match");
}

#[test]
fn test_template_rendering() {
    // Create a temporary directory for test templates
    let temp_dir = tempfile::tempdir().unwrap();
    let template_path = temp_dir.path();
    
    // Create a template file
    let template_content = "Hello, {{name}}!";
    let template_file_path = template_path.join("greeting.tmpl");
    fs::write(&template_file_path, template_content).unwrap();
    
    // Initialize the template manager
    let mut template_manager = TemplateManager::new(template_path).unwrap();
    
    // Create parameters for rendering
    let mut params = HashMap::new();
    params.insert("name".to_string(), "World".to_string());
    
    // Render the template
    let rendered = template_manager.render_template("greeting.tmpl", &params);
    
    assert!(rendered.is_ok(), "Template rendering failed");
    assert_eq!(rendered.unwrap(), "Hello, World!", "Rendered content doesn't match expected output");
}

#[test]
fn test_missing_template() {
    // Create a temporary directory for test templates
    let temp_dir = tempfile::tempdir().unwrap();
    let template_path = temp_dir.path();
    
    // Initialize the template manager
    let mut template_manager = TemplateManager::new(template_path).unwrap();
    
    // Try to load a non-existent template
    let result = template_manager.load_template("non_existent.tmpl");
    
    assert!(result.is_err(), "Expected error when loading non-existent template");
}

#[test]
fn test_multiple_placeholder_replacement() {
    // Create a temporary directory for test templates
    let temp_dir = tempfile::tempdir().unwrap();
    let template_path = temp_dir.path();
    
    // Create a template file with multiple placeholders
    let template_content = "{{greeting}}, {{name}}! Welcome to {{project}}.";
    let template_file_path = template_path.join("complex.tmpl");
    fs::write(&template_file_path, template_content).unwrap();
    
    // Initialize the template manager
    let mut template_manager = TemplateManager::new(template_path).unwrap();
    
    // Create parameters for rendering
    let mut params = HashMap::new();
    params.insert("greeting".to_string(), "Hello".to_string());
    params.insert("name".to_string(), "Developer".to_string());
    params.insert("project".to_string(), "Kumeo".to_string());
    
    // Render the template
    let rendered = template_manager.render_template("complex.tmpl", &params);
    
    assert!(rendered.is_ok(), "Template rendering failed");
    assert_eq!(
        rendered.unwrap(), 
        "Hello, Developer! Welcome to Kumeo.", 
        "Rendered content with multiple placeholders doesn't match expected output"
    );
}

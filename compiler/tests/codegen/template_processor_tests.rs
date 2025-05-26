use anyhow::Result;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use tempfile::tempdir;
use tera::Tera;

use kumeo_compiler::codegen::template_processor::{
    create_base_context,
    ensure_dir,
    file_contains,
    process_template_dir,
    render_template_string,
};

#[test]
fn test_create_base_context() {
    let context = create_base_context("test_workflow");
    assert_eq!(context.get("workflow_name").unwrap(), &"test_workflow".to_string());
    assert!(context.get("current_year").is_some());
}

#[test]
fn test_ensure_dir() -> Result<()> {
    let temp_dir = tempdir()?;
    let test_dir = temp_dir.path().join("test_dir");
    
    // Directory should not exist initially
    assert!(!test_dir.exists());
    
    // Create directory
    ensure_dir(&test_dir)?;
    
    // Directory should exist now
    assert!(test_dir.exists());
    assert!(test_dir.is_dir());
    
    // Calling again should not fail
    ensure_dir(&test_dir)?;
    
    Ok(())
}

#[test]
fn test_file_contains() -> Result<()> {
    let temp_dir = tempdir()?;
    let test_file = temp_dir.path().join("test.txt");
    
    // Create test file
    let mut file = File::create(&test_file)?;
    writeln!(file, "This is a test file")?;
    writeln!(file, "With multiple lines")?;
    writeln!(file, "And some more content")?;
    
    // Test file_contains function
    assert!(file_contains(&test_file, "test file")?);
    assert!(file_contains(&test_file, "multiple lines")?);
    assert!(!file_contains(&test_file, "nonexistent")?);
    
    Ok(())
}

#[test]
fn test_render_template_string() -> Result<()> {
    let mut context = tera::Context::new();
    context.insert("name", "Test");
    
    let template = "Hello, {{ name }}!";
    let result = render_template_string(template, &context, &Tera::default())?;
    
    assert_eq!(result, "Hello, Test!");
    
    Ok(())
}

#[test]
fn test_process_template_dir() -> Result<()> {
    // Create a temporary directory for the test
    let temp_dir = tempdir()?;
    let template_dir = temp_dir.path().join("templates");
    let output_dir = temp_dir.path().join("output");
    
    // Create a simple template structure
    fs::create_dir_all(template_dir.join("subdir"))?;
    
    // Create template files
    let mut file = File::create(template_dir.join("test.txt.tera"))?;
    writeln!(file, "Hello, {{ name }}!")?;
    
    let mut file = File::create(template_dir.join("subdir/another.txt.tera"))?;
    writeln!(file, "This is a test")?;
    
    // Create a non-template file that should be copied as-is
    let mut file = File::create(template_dir.join("config.json"))?;
    writeln!(file, "{{\"key\": \"value\"}}")?;
    
    // Create context
    let mut context = tera::Context::new();
    context.insert("name", "World");
    
    // Process templates
    let tera = Tera::new(&format!("{}/**/*.tera", template_dir.display()))?;
    process_template_dir(&template_dir, &output_dir, &context, &tera, &[])?;
    
    // Verify output files
    let output_file = output_dir.join("test.txt");
    assert!(output_file.exists());
    assert_eq!(fs::read_to_string(output_file)?, "Hello, World!\n");
    
    let subdir_file = output_dir.join("subdir/another.txt");
    assert!(subdir_file.exists());
    assert_eq!(fs::read_to_string(subdir_file)?, "This is a test\n");
    
    // Verify non-template file was copied
    let config_file = output_dir.join("config.json");
    assert!(config_file.exists());
    assert_eq!(fs::read_to_string(config_file)?, "{\"key\": \"value\"}\n");
    
    Ok(())
}

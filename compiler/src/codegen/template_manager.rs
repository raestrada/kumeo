use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TemplateError {
    #[error("Template not found: {0}")]
    NotFound(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Template rendering error: {0}")]
    Rendering(String),
}

pub type Result<T> = std::result::Result<T, TemplateError>;

/// Manages code generation templates for different languages
pub struct TemplateManager {
    template_root: PathBuf,
    templates: HashMap<String, String>,
}

impl TemplateManager {
    /// Create a new template manager with the given root directory
    pub fn new<P: AsRef<Path>>(template_root: P) -> Result<Self> {
        let template_root = template_root.as_ref().to_path_buf();
        if !template_root.exists() {
            return Err(TemplateError::NotFound(format!(
                "Template root directory not found: {}",
                template_root.display()
            )));
        }
        
        Ok(Self {
            template_root,
            templates: HashMap::new(),
        })
    }
    
    /// Load a template by its path relative to the template root
    pub fn load_template(&mut self, relative_path: &str) -> Result<&str> {
        if !self.templates.contains_key(relative_path) {
            let full_path = self.template_root.join(relative_path);
            if !full_path.exists() {
                return Err(TemplateError::NotFound(format!(
                    "Template not found: {}",
                    full_path.display()
                )));
            }
            
            let template_content = fs::read_to_string(&full_path)?;
            self.templates.insert(relative_path.to_string(), template_content);
        }
        
        Ok(self.templates.get(relative_path).unwrap())
    }
    
    /// Render a template with the given parameters
    pub fn render_template(&mut self, template_path: &str, params: &HashMap<String, String>) -> Result<String> {
        let template = self.load_template(template_path)?;
        let mut result = template.to_string();
        
        // Simple placeholder replacement
        for (key, value) in params {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, value);
        }
        
        Ok(result)
    }
}

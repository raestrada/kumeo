use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;
use crate::{debug, info, warn, error, trace};

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
        debug!(path = %template_root.display(), "Creating template manager");
        
        if !template_root.exists() {
            error!(path = %template_root.display(), "Template root directory not found");
            return Err(TemplateError::NotFound(format!(
                "Template root directory not found: {}",
                template_root.display()
            )));
        }
        
        info!(path = %template_root.display(), "Template manager initialized");
        
        Ok(Self {
            template_root,
            templates: HashMap::new(),
        })
    }
    
    /// Load a template by its path relative to the template root
    pub fn load_template(&mut self, relative_path: &str) -> Result<&str> {
        debug!(template = %relative_path, "Loading template");
        
        if !self.templates.contains_key(relative_path) {
            let full_path = self.template_root.join(relative_path);
            trace!(path = %full_path.display(), "Full template path");
            
            if !full_path.exists() {
                error!(path = %full_path.display(), "Template not found");
                return Err(TemplateError::NotFound(format!(
                    "Template not found: {}",
                    full_path.display()
                )));
            }
            
            match fs::read_to_string(&full_path) {
                Ok(template_content) => {
                    debug!(template = %relative_path, size = template_content.len(), "Template loaded successfully");
                    self.templates.insert(relative_path.to_string(), template_content);
                },
                Err(e) => {
                    error!(error = %e, path = %full_path.display(), "Failed to read template file");
                    return Err(TemplateError::Io(e));
                }
            }
        }
        
        trace!(template = %relative_path, "Retrieving template from cache");
        self.templates.get(relative_path).map(|s| s.as_str()).ok_or_else(|| {
            let err = format!("Template not found in cache: {}", relative_path);
            error!(template = %relative_path, "Template missing from cache");
            TemplateError::NotFound(err)
        })
    }
    
    /// Render a template with the given parameters
    pub fn render_template(&mut self, template_path: &str, params: &HashMap<String, String>) -> Result<String> {
        info!(template = %template_path, param_count = params.len(), "Rendering template");
        let template = self.load_template(template_path)?;
        let mut result = template.to_string();
        
        // Log parameter keys for debugging (values might contain sensitive info)
        if !params.is_empty() {
            let keys: Vec<&String> = params.keys().collect();
            trace!(keys = ?keys, "Template parameters");
        }
        
        // Simple placeholder replacement
        for (key, value) in params {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, value);
        }
        
        debug!(template = %template_path, size_before = template.len(), size_after = result.len(), "Template rendered");
        Ok(result)
    }
}

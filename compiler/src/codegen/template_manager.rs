use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::str::FromStr;
use thiserror::Error;
use tera::{Tera, Context as TeraContext, Value as TeraValue, to_value, from_value};
use serde::{Serialize, Deserialize};
use heck::ToSnakeCase;
use chrono;
use crate::{debug, info, error, trace};

/// Helper to convert any serializable type to Tera's Value
pub fn to_tera_value<T: Serialize>(value: &T) -> Result<TeraValue, tera::Error> {
    to_value(value).map_err(Into::into)
}

/// Helper to convert from Tera's Value to a concrete type
pub fn from_tera_value<T: for<'de> Deserialize<'de>>(value: TeraValue) -> Result<T, tera::Error> {
    from_value(value).map_err(Into::into)
}

#[derive(Error, Debug)]
pub enum TemplateError {
    #[error("Template not found: {0}")]
    NotFound(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Template rendering error: {0}")]
    Rendering(#[from] tera::Error),
    #[error("Template compilation error: {0}")]
    Compilation(String),
}

pub type Result<T> = std::result::Result<T, TemplateError>;

/// Manages code generation templates for different languages using Tera
#[derive(Clone)]
pub struct TemplateManager {
    tera: Arc<Tera>,
    template_root: PathBuf,
    
    // Common context shared across all template renders
    common_context: TeraContext,
}

impl TemplateManager {
    /// Create a new template manager with the given root directory
    pub fn new<P: AsRef<Path>>(template_root: P) -> Result<Self> {
        let template_root = template_root.as_ref().to_path_buf();
        debug!(path = %template_root.display(), "Creating template manager");
        
        // Initialize Tera with the template directory
        let mut tera = match Tera::new(template_root.to_str().unwrap_or_default()) {
            Ok(t) => t,
            Err(e) => {
                error!(error = %e, "Failed to parse templates");
                return Err(TemplateError::Compilation(e.to_string()));
            }
        };
        
        // Register custom components
        Self::register_custom_components(&mut tera)?;
        
        // Create common context
        let mut common_context = TeraContext::new();
        common_context.insert("version", env!("CARGO_PKG_VERSION"));
        common_context.insert("project_name", "kumeo");
        
        info!(path = %template_root.display(), "Template manager initialized");
        
        Ok(Self {
            tera: Arc::new(tera),
            template_root,
            common_context,
        })
    }
    
    /// Render a template with the given parameters
    pub fn render_template(&self, template_path: &str, params: &HashMap<String, String>) -> Result<String> {
        info!(template = %template_path, param_count = params.len(), "Rendering template");
        
        // Log parameter keys for debugging (values might contain sensitive info)
        if !params.is_empty() {
            let keys: Vec<&String> = params.keys().collect();
            trace!(keys = ?keys, "Template parameters");
        }
        
        // Convert HashMap to Tera Context
        let mut context = TeraContext::new();
        for (key, value) in params {
            context.insert(key, value);
        }
        
        // Render the template
        let result = self.tera.render(template_path, &context)?;
        debug!(template = %template_path, size = result.len(), "Template rendered successfully");
        Ok(result)
    }
    
    /// Render a template from a string with the given parameters
    pub fn render_string(&self, template: &str, params: &HashMap<String, String>) -> Result<String> {
        // Convert HashMap to Tera Context
        let mut context = TeraContext::new();
        for (key, value) in params {
            context.insert(key, value);
        }
        
        // Render the template string
        self.tera.render_str(template, &context)
            .map_err(Into::into)
    }
    
    /// Register custom filters, functions, and testers for Tera
    fn register_custom_components(tera: &mut Tera) -> Result<()> {
        // Register custom filters
        tera.register_filter("to_snake_case", |value, _| {
            if let TeraValue::String(s) = value {
                Ok(TeraValue::String(s.to_snake_case()))
            } else {
                Err(tera::Error::msg("Filter `to_snake_case` was used on a value that isn't a string"))
            }
        });
        
        // Register custom functions
        tera.register_function("now", |_| {
            let now = chrono::Local::now();
            Ok(TeraValue::String(now.to_rfc3339()))
        });
        
        // Register custom testers
        tera.register_tester("containing", |args| {
            if args.len() != 2 {
                return Ok(false);
            }
            
            let (collection, value) = (&args[0], &args[1]);
            
            if let Some(arr) = collection.as_array() {
                Ok(arr.contains(value))
            } else if let Some(map) = collection.as_object() {
                if let Some(key) = value.as_str() {
                    Ok(map.contains_key(key))
                } else {
                    Ok(false)
                }
            } else if let (Some(collection_str), Some(value_str)) = (collection.as_str(), value.as_str()) {
                Ok(collection_str.contains(value_str))
            } else {
                Ok(false)
            }
        });
        
        // Enable auto-escaping for HTML/XML/Markdown templates
        tera.autoescape_on(vec![".html", ".htm", ".xml", ".md"]);
        
        Ok(())
    }
    
    /// Render a template with a serializable context
    pub fn render_with_serializable<T: Serialize>(
        &self, 
        template_path: &str, 
        context: &T
    ) -> Result<String> {
        let context_value = to_tera_value(context)?;
        let mut tera_context = self.common_context.clone();
        
        if let TeraValue::Object(map) = context_value {
            for (key, value) in map {
                tera_context.insert(&key, &value);
            }
        }
        
        self.tera.render(template_path, &tera_context)
            .map_err(Into::into)
    }
    
    /// Get a reference to the underlying Tera instance
    pub fn tera(&self) -> &Tera {
        &self.tera
    }
    
    /// Get a reference to the template root directory
    pub fn template_root(&self) -> &Path {
        &self.template_root
    }
}

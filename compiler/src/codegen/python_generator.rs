use std::collections::HashMap;
use std::path::{Path, PathBuf};
use crate::ast::{Agent, Workflow};
use crate::codegen::template_manager::{TemplateManager, Result, TemplateError};
use tracing::{debug, info, warn, error, trace, instrument};

pub struct PythonGenerator {
    template_manager: TemplateManager,
    output_dir: PathBuf,
}

impl PythonGenerator {
    pub fn new<P: AsRef<Path>>(template_root: P, output_dir: P) -> Result<Self> {
        debug!(template_path = ?template_root.as_ref().display(), "Initializing Python generator");
        let template_manager = TemplateManager::new(template_root.as_ref().join("python"))?;
        let output_dir = output_dir.as_ref().join("python");
        
        // Create output directory if it doesn't exist
        if !output_dir.exists() {
            debug!(path = ?output_dir.display(), "Creating Python output directory");
            std::fs::create_dir_all(&output_dir)?;
        }
        
        info!("Python generator initialized");
        
        Ok(Self {
            template_manager,
            output_dir,
        })
    }
    
    /// Generate Python code for an ML Model agent
    #[instrument(skip(self, agent, workflow), fields(agent_id = ?agent.id, workflow_name = %workflow.name))]
    pub fn generate_ml_model_agent(&mut self, agent: &Agent, workflow: &Workflow) -> Result<PathBuf> {
        info!("Generating Python code for ML Model agent");
        let agent_id = agent.id.as_ref().ok_or_else(|| {
            error!("Agent is missing ID");
            TemplateError::Rendering("Agent is missing ID".to_string())
        })?;
        
        // Extract agent configuration
        let mut params = HashMap::new();
        params.insert("agent_id".to_string(), agent_id.clone());
        params.insert("workflow_name".to_string(), workflow.name.clone());
        
        // Find the model_path parameter
        let model_path = self.get_agent_param(agent, "model_path")
            .unwrap_or_else(|| {
                debug!("No model_path specified, using default: model.pkl");
                "model.pkl".to_string()
            });
        debug!(model_path = %model_path, "Using ML model path");
        params.insert("model_path".to_string(), model_path);
        
        // Extract source and target topics
        if let Some(source) = &workflow.source {
            params.insert("input_topic".to_string(), self.extract_topic(source));
        }
        
        if let Some(target) = &workflow.target {
            params.insert("output_topic".to_string(), self.extract_topic(target));
        }
        
        // Render the template
        debug!("Rendering ML Model agent template");
        let code = self.template_manager.render_template("agents/ml_model.py.tmpl", &params)?;
        
        // Create the output file
        let output_file = self.output_dir
            .join("src")
            .join("agents")
            .join(format!("{}.py", agent_id.to_lowercase()));
        
        debug!(path = ?output_file.display(), "Creating directory structure for ML Model agent");
        match std::fs::create_dir_all(output_file.parent().unwrap()) {
            Ok(_) => {},
            Err(e) => {
                error!(error = %e, path = ?output_file.parent().unwrap().display(), "Failed to create directory structure");
                return Err(TemplateError::Io(e));
            }
        }
        
        debug!(path = ?output_file.display(), "Writing generated ML Model agent code");
        match std::fs::write(&output_file, &code) {
            Ok(_) => {
                info!(agent_id = %agent_id, path = %output_file.display(), "Successfully generated ML Model agent code");
                Ok(output_file)
            },
            Err(e) => {
                error!(error = %e, path = %output_file.display(), "Failed to write ML Model agent code");
                Err(TemplateError::Io(e))
            }
        }
    }
    
    /// Generate Python code for a Bayesian Network agent
    #[instrument(skip(self, agent, workflow), fields(agent_id = ?agent.id, workflow_name = %workflow.name))]
    pub fn generate_bayesian_network_agent(&mut self, agent: &Agent, workflow: &Workflow) -> Result<PathBuf> {
        info!("Generating Python code for Bayesian Network agent");
        let agent_id = agent.id.as_ref().ok_or_else(|| {
            error!("Agent is missing ID");
            TemplateError::Rendering("Agent is missing ID".to_string())
        })?;
        
        // Extract agent configuration
        let mut params = HashMap::new();
        params.insert("agent_id".to_string(), agent_id.clone());
        params.insert("workflow_name".to_string(), workflow.name.clone());
        
        // Find the network_path parameter
        let network_path = self.get_agent_param(agent, "network_path")
            .unwrap_or_else(|| "network.bn".to_string());
        params.insert("network_path".to_string(), network_path);
        
        // Extract source and target topics
        if let Some(source) = &workflow.source {
            params.insert("input_topic".to_string(), self.extract_topic(source));
        }
        
        if let Some(target) = &workflow.target {
            params.insert("output_topic".to_string(), self.extract_topic(target));
        }
        
        // Render the template
        let code = self.template_manager.render_template("agents/bayesian_network.py.tmpl", &params)?;
        
        // Create the output file
        let output_file = self.output_dir
            .join("src")
            .join("agents")
            .join(format!("{}.py", agent_id.to_lowercase()));
        
        std::fs::create_dir_all(output_file.parent().unwrap())?;
        std::fs::write(&output_file, code)?;
        
        Ok(output_file)
    }
    
    /// Generate Python code for a Decision Matrix agent
    pub fn generate_decision_matrix_agent(&mut self, agent: &Agent, workflow: &Workflow) -> Result<PathBuf> {
        let agent_id = agent.id.as_ref().ok_or_else(|| {
            TemplateError::Rendering("Agent is missing ID".to_string())
        })?;
        
        // Extract agent configuration
        let mut params = HashMap::new();
        params.insert("agent_id".to_string(), agent_id.clone());
        params.insert("workflow_name".to_string(), workflow.name.clone());
        
        // Find the matrix_definition parameter
        let matrix_def = self.get_agent_param(agent, "matrix_definition")
            .unwrap_or_else(|| "matrix.dmx".to_string());
        params.insert("matrix_definition".to_string(), matrix_def);
        
        // Extract source and target topics
        if let Some(source) = &workflow.source {
            params.insert("input_topic".to_string(), self.extract_topic(source));
        }
        
        if let Some(target) = &workflow.target {
            params.insert("output_topic".to_string(), self.extract_topic(target));
        }
        
        // Render the template
        let code = self.template_manager.render_template("agents/decision_matrix.py.tmpl", &params)?;
        
        // Create the output file
        let output_file = self.output_dir
            .join("src")
            .join("agents")
            .join(format!("{}.py", agent_id.to_lowercase()));
        
        std::fs::create_dir_all(output_file.parent().unwrap())?;
        std::fs::write(&output_file, code)?;
        
        Ok(output_file)
    }
    
    /// Helper to get a parameter from an agent's config
    #[instrument(skip(self, agent), fields(param_name = %param_name, agent_id = ?agent.id))]
    fn get_agent_param(&self, agent: &Agent, param_name: &str) -> Option<String> {
        trace!("Retrieving parameter from agent config");
        for arg in &agent.config {
            match arg {
                crate::ast::Argument::Named(name, value) => {
                    if name == param_name {
                        match value {
                            crate::ast::Value::String(s) => {
                                trace!(param = %param_name, value = %s, "Found string parameter in agent config");
                                return Some(s.clone());
                            },
                            _ => {
                                let val_str = format!("{:?}", value);
                                trace!(param = %param_name, value = %val_str, "Found non-string parameter in agent config");
                                return Some(val_str);
                            }
                        }
                    }
                },
                _ => continue,
            }
        }
        trace!(param = %param_name, "Parameter not found in agent config");
        None
    }
    
    /// Extract topic name from a source or target
    #[instrument(skip(self, source_or_target))]
    fn extract_topic(&self, source_or_target: &impl std::fmt::Debug) -> String {
        trace!("Extracting topic name from source or target");
        // This is a simplified version - actual implementation would need to match on the enum variants
        let raw_repr = format!("{:?}", source_or_target);
        let topic = raw_repr.replace("NATS(", "").replace(")", "").replace("\"", "");
        trace!(raw = %raw_repr, topic = %topic, "Extracted topic name");
        topic
    }
}

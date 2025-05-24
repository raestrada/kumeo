use std::collections::HashMap;
use std::path::{Path, PathBuf};
use crate::ast::{Agent, Workflow};
use crate::codegen::template_manager::{TemplateManager, Result, TemplateError};
use tracing::{debug, info, warn, error, trace, instrument};

pub struct RustGenerator {
    template_manager: TemplateManager,
    output_dir: PathBuf,
}

impl RustGenerator {
    pub fn new<P: AsRef<Path>>(template_root: P, output_dir: P) -> Result<Self> {
        debug!(template_path = ?template_root.as_ref().display(), "Initializing Rust generator");
        let template_manager = TemplateManager::new(template_root.as_ref().join("rust"))?;
        let output_dir = output_dir.as_ref().join("rust");
        
        // Create output directory if it doesn't exist
        if !output_dir.exists() {
            debug!(path = ?output_dir.display(), "Creating Rust output directory");
            std::fs::create_dir_all(&output_dir)?;
        }
        
        info!("Rust generator initialized");
        
        Ok(Self {
            template_manager,
            output_dir,
        })
    }
    
    /// Generate Rust code for an LLM agent
    #[instrument(skip(self, agent, workflow), fields(agent_id = ?agent.id, workflow_name = %workflow.name))]
    pub fn generate_llm_agent(&mut self, agent: &Agent, workflow: &Workflow) -> Result<PathBuf> {
        info!("Generating Rust code for LLM agent");
        let agent_id = agent.id.as_ref().ok_or_else(|| {
            error!("Agent is missing ID");
            TemplateError::Rendering("Agent is missing ID".to_string())
        })?;
        
        // Extract agent configuration
        let mut params = HashMap::new();
        params.insert("agent_id".to_string(), agent_id.clone());
        params.insert("workflow_name".to_string(), workflow.name.clone());
        
        // Find the engine parameter
        let engine = self.get_agent_param(agent, "engine")
            .unwrap_or_else(|| {
                debug!("No engine specified, using default: ollama/llama3");
                "ollama/llama3".to_string()
            });
        debug!(engine = %engine, "Using LLM engine");
        params.insert("engine".to_string(), engine);
        
        // Find the prompt parameter
        let prompt = self.get_agent_param(agent, "prompt")
            .unwrap_or_else(|| "".to_string());
        params.insert("prompt".to_string(), prompt);
        
        // Extract source and target topics
        if let Some(source) = &workflow.source {
            params.insert("input_topic".to_string(), self.extract_topic(source));
        }
        
        if let Some(target) = &workflow.target {
            params.insert("output_topic".to_string(), self.extract_topic(target));
        }
        
        // Render the LLM agent template
        debug!("Rendering LLM agent template");
        let template_content = self.template_manager.render_template("agents/llm.rs.tmpl", &params)?;
        
        // Write the generated code to a file
        let output_file = self.output_dir
            .join("src")
            .join("agents")
            .join(format!("{}.rs", agent_id.to_lowercase()));
        debug!(path = %output_file.display(), "Writing generated LLM agent code");
        
        match std::fs::write(&output_file, &template_content) {
            Ok(_) => {
                info!(agent_id = %agent_id, path = %output_file.display(), "Successfully generated LLM agent code");
                Ok(output_file)
            },
            Err(e) => {
                error!(error = %e, path = %output_file.display(), "Failed to write LLM agent code");
                Err(TemplateError::Io(e))
            }
        }
    }
    
    /// Generate Rust code for a router agent
    #[instrument(skip(self, agent, workflow), fields(agent_id = ?agent.id, workflow_name = %workflow.name))]
    pub fn generate_router_agent(&mut self, agent: &Agent, workflow: &Workflow) -> Result<PathBuf> {
        info!("Generating Rust code for Router agent");
        let agent_id = agent.id.as_ref().ok_or_else(|| {
            error!("Agent is missing ID");
            TemplateError::Rendering("Agent is missing ID".to_string())
        })?;
        
        // Extract agent configuration
        let mut params = HashMap::new();
        params.insert("agent_id".to_string(), agent_id.clone());
        params.insert("workflow_name".to_string(), workflow.name.clone());
        
        // Find the routing_rules parameter
        let routing_rules = self.get_agent_param(agent, "routing_rules")
            .unwrap_or_else(|| "{}".to_string());
        params.insert("routing_rules".to_string(), routing_rules);
        
        // Extract source and target topics
        if let Some(source) = &workflow.source {
            params.insert("input_topic".to_string(), self.extract_topic(source));
        }
        
        if let Some(target) = &workflow.target {
            params.insert("output_topic".to_string(), self.extract_topic(target));
        }
        
        // Render the template
        debug!("Rendering Router agent template");
        let code = self.template_manager.render_template("agents/router.rs.tmpl", &params)?;
        
        // Create the output file
        let output_file = self.output_dir
            .join("src")
            .join("agents")
            .join(format!("{}.rs", agent_id.to_lowercase()));
        
        debug!(path = ?output_file.display(), "Creating directory structure for Router agent");
        match std::fs::create_dir_all(output_file.parent().unwrap()) {
            Ok(_) => {},
            Err(e) => {
                error!(error = %e, path = ?output_file.parent().unwrap().display(), "Failed to create directory structure");
                return Err(TemplateError::Io(e));
            }
        }
        
        debug!(path = ?output_file.display(), "Writing generated Router agent code");
        match std::fs::write(&output_file, &code) {
            Ok(_) => {
                info!(agent_id = %agent_id, path = %output_file.display(), "Successfully generated Router agent code");
                Ok(output_file)
            },
            Err(e) => {
                error!(error = %e, path = %output_file.display(), "Failed to write Router agent code");
                Err(TemplateError::Io(e))
            }
        }
    }
    
    // Similar methods for other agent types
    pub fn generate_aggregator_agent(&mut self, _agent: &Agent, _workflow: &Workflow) -> Result<PathBuf> {
        // Implementation similar to other agent generators
        Ok(PathBuf::new()) // Placeholder
    }
    
    pub fn generate_rule_engine_agent(&mut self, _agent: &Agent, _workflow: &Workflow) -> Result<PathBuf> {
        // Implementation similar to other agent generators
        Ok(PathBuf::new()) // Placeholder
    }
    
    pub fn generate_data_normalizer_agent(&mut self, _agent: &Agent, _workflow: &Workflow) -> Result<PathBuf> {
        // Implementation similar to other agent generators
        Ok(PathBuf::new()) // Placeholder
    }
    
    pub fn generate_missing_value_handler_agent(&mut self, _agent: &Agent, _workflow: &Workflow) -> Result<PathBuf> {
        // Implementation similar to other agent generators
        Ok(PathBuf::new()) // Placeholder
    }
    
    pub fn generate_human_in_loop_backend(&mut self, _agent: &Agent, _workflow: &Workflow) -> Result<PathBuf> {
        // Implementation similar to other agent generators
        Ok(PathBuf::new()) // Placeholder
    }
    
    pub fn generate_custom_agent(&mut self, _agent: &Agent, _workflow: &Workflow) -> Result<PathBuf> {
        // Implementation similar to other agent generators
        Ok(PathBuf::new()) // Placeholder
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

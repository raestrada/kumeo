use std::collections::HashMap;
use std::path::{Path, PathBuf};
use crate::ast::{Agent, AgentType, Workflow, Source, Target};
use crate::codegen::template_manager::{TemplateManager, Result, TemplateError};

pub struct RustGenerator {
    template_manager: TemplateManager,
    output_dir: PathBuf,
}

impl RustGenerator {
    pub fn new<P: AsRef<Path>>(template_root: P, output_dir: P) -> Result<Self> {
        let template_manager = TemplateManager::new(template_root.as_ref().join("rust"))?;
        let output_dir = output_dir.as_ref().join("rust");
        
        // Create output directory if it doesn't exist
        if !output_dir.exists() {
            std::fs::create_dir_all(&output_dir)?;
        }
        
        Ok(Self {
            template_manager,
            output_dir,
        })
    }
    
    /// Generate Rust code for an LLM agent
    pub fn generate_llm_agent(&mut self, agent: &Agent, workflow: &Workflow) -> Result<PathBuf> {
        let agent_id = agent.id.as_ref().ok_or_else(|| {
            TemplateError::Rendering("Agent is missing ID".to_string())
        })?;
        
        // Extract agent configuration
        let mut params = HashMap::new();
        params.insert("agent_id".to_string(), agent_id.clone());
        params.insert("workflow_name".to_string(), workflow.name.clone());
        
        // Find the engine parameter
        let engine = self.get_agent_param(agent, "engine")
            .unwrap_or_else(|| "ollama/llama3".to_string());
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
        
        // Render the template
        let code = self.template_manager.render_template("agents/llm.rs.tmpl", &params)?;
        
        // Create the output file
        let output_file = self.output_dir
            .join("src")
            .join("agents")
            .join(format!("{}.rs", agent_id.to_lowercase()));
        
        std::fs::create_dir_all(output_file.parent().unwrap())?;
        std::fs::write(&output_file, code)?;
        
        Ok(output_file)
    }
    
    /// Generate Rust code for a router agent
    pub fn generate_router_agent(&mut self, agent: &Agent, workflow: &Workflow) -> Result<PathBuf> {
        let agent_id = agent.id.as_ref().ok_or_else(|| {
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
        let code = self.template_manager.render_template("agents/router.rs.tmpl", &params)?;
        
        // Create the output file
        let output_file = self.output_dir
            .join("src")
            .join("agents")
            .join(format!("{}.rs", agent_id.to_lowercase()));
        
        std::fs::create_dir_all(output_file.parent().unwrap())?;
        std::fs::write(&output_file, code)?;
        
        Ok(output_file)
    }
    
    // Similar methods for other agent types
    pub fn generate_aggregator_agent(&mut self, agent: &Agent, workflow: &Workflow) -> Result<PathBuf> {
        // Implementation similar to other agent generators
        Ok(PathBuf::new()) // Placeholder
    }
    
    pub fn generate_rule_engine_agent(&mut self, agent: &Agent, workflow: &Workflow) -> Result<PathBuf> {
        // Implementation similar to other agent generators
        Ok(PathBuf::new()) // Placeholder
    }
    
    pub fn generate_data_normalizer_agent(&mut self, agent: &Agent, workflow: &Workflow) -> Result<PathBuf> {
        // Implementation similar to other agent generators
        Ok(PathBuf::new()) // Placeholder
    }
    
    pub fn generate_missing_value_handler_agent(&mut self, agent: &Agent, workflow: &Workflow) -> Result<PathBuf> {
        // Implementation similar to other agent generators
        Ok(PathBuf::new()) // Placeholder
    }
    
    pub fn generate_human_in_loop_backend(&mut self, agent: &Agent, workflow: &Workflow) -> Result<PathBuf> {
        // Implementation similar to other agent generators
        Ok(PathBuf::new()) // Placeholder
    }
    
    pub fn generate_custom_agent(&mut self, agent: &Agent, workflow: &Workflow) -> Result<PathBuf> {
        // Implementation similar to other agent generators
        Ok(PathBuf::new()) // Placeholder
    }
    
    /// Helper to get a parameter from an agent's config
    fn get_agent_param(&self, agent: &Agent, param_name: &str) -> Option<String> {
        for arg in &agent.config {
            match arg {
                crate::ast::Argument::Named(name, value) => {
                    if name == param_name {
                        match value {
                            crate::ast::Value::String(s) => return Some(s.clone()),
                            _ => return Some(format!("{:?}", value)),
                        }
                    }
                },
                _ => continue,
            }
        }
        None
    }
    
    /// Extract topic name from a source or target
    fn extract_topic(&self, source_or_target: &impl std::fmt::Debug) -> String {
        // This is a simplified version - actual implementation would need to match on the enum variants
        format!("{:?}", source_or_target).replace("NATS(", "").replace(")", "").replace("\"", "")
    }
}

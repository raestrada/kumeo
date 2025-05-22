use std::collections::HashMap;
use std::path::{Path, PathBuf};
use crate::ast::{Agent, AgentType, Workflow};
use crate::codegen::template_manager::{TemplateManager, Result, TemplateError};

pub struct PythonGenerator {
    template_manager: TemplateManager,
    output_dir: PathBuf,
}

impl PythonGenerator {
    pub fn new<P: AsRef<Path>>(template_root: P, output_dir: P) -> Result<Self> {
        let template_manager = TemplateManager::new(template_root.as_ref().join("python"))?;
        let output_dir = output_dir.as_ref().join("python");
        
        // Create output directory if it doesn't exist
        if !output_dir.exists() {
            std::fs::create_dir_all(&output_dir)?;
        }
        
        Ok(Self {
            template_manager,
            output_dir,
        })
    }
    
    /// Generate Python code for an ML Model agent
    pub fn generate_ml_model_agent(&mut self, agent: &Agent, workflow: &Workflow) -> Result<PathBuf> {
        let agent_id = agent.id.as_ref().ok_or_else(|| {
            TemplateError::Rendering("Agent is missing ID".to_string())
        })?;
        
        // Extract agent configuration
        let mut params = HashMap::new();
        params.insert("agent_id".to_string(), agent_id.clone());
        params.insert("workflow_name".to_string(), workflow.name.clone());
        
        // Find the model_path parameter
        let model_path = self.get_agent_param(agent, "model_path")
            .unwrap_or_else(|| "model.pkl".to_string());
        params.insert("model_path".to_string(), model_path);
        
        // Extract source and target topics
        if let Some(source) = &workflow.source {
            params.insert("input_topic".to_string(), self.extract_topic(source));
        }
        
        if let Some(target) = &workflow.target {
            params.insert("output_topic".to_string(), self.extract_topic(target));
        }
        
        // Render the template
        let code = self.template_manager.render_template("agents/ml_model.py.tmpl", &params)?;
        
        // Create the output file
        let output_file = self.output_dir
            .join("src")
            .join("agents")
            .join(format!("{}.py", agent_id.to_lowercase()));
        
        std::fs::create_dir_all(output_file.parent().unwrap())?;
        std::fs::write(&output_file, code)?;
        
        Ok(output_file)
    }
    
    /// Generate Python code for a Bayesian Network agent
    pub fn generate_bayesian_network_agent(&mut self, agent: &Agent, workflow: &Workflow) -> Result<PathBuf> {
        let agent_id = agent.id.as_ref().ok_or_else(|| {
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

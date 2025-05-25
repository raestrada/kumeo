use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use serde::Serialize;
use anyhow::Context;
use crate::ast::{Agent, Workflow, AgentType};
use crate::codegen::template_manager::{TemplateManager, Result, TemplateError};
use tracing::{debug, info, warn, error, trace, instrument};

/// Context for Rust code generation
#[derive(Debug, Serialize)]
struct RustContext {
    project_name: String,
    version: String,
    agents: Vec<AgentContext>,
    workflows: Vec<WorkflowContext>,
}

/// Context for agent code generation
#[derive(Debug, Serialize)]
struct AgentContext {
    id: String,
    agent_type: AgentType,
    config: serde_json::Value,
    input_topic: Option<String>,
    output_topic: Option<String>,
}

/// Context for workflow code generation
#[derive(Debug, Serialize)]
struct WorkflowContext {
    name: String,
    agents: Vec<String>,
}

pub struct RustGenerator {
    template_manager: TemplateManager,
    output_dir: PathBuf,
    context: RustContext,
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
        
        // Initialize context with default values
        let context = RustContext {
            project_name: "kumeo_agent".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            agents: Vec::new(),
            workflows: Vec::new(),
        };
        
        info!("Rust generator initialized");
        
        Ok(Self {
            template_manager,
            output_dir,
            context,
        })
    }
    
    /// Generate Rust code for an LLM agent
    #[instrument(skip(self, agent, workflow), fields(agent_id = ?agent.id, workflow_name = %workflow.name))]
    pub fn generate_llm_agent(&mut self, agent: &Agent, workflow: &Workflow) -> Result<PathBuf> {
        info!("Generating Rust code for LLM agent");
        let agent_id = agent.id.as_ref().ok_or_else(|| {
            let msg = "Agent is missing ID".to_string();
            error!("{}", msg);
            TemplateError::Rendering(tera::Error::msg("Agent is missing ID".to_string()))
        })?;
        
        // Create agent context
        let agent_ctx = AgentContext {
            id: agent_id.clone(),
            agent_type: agent.agent_type.clone(),
            config: serde_json::to_value(agent.config.clone()).unwrap_or_default(),
            input_topic: workflow.source.as_ref().map(|s| self.extract_topic(s)),
            output_topic: workflow.target.as_ref().map(|t| self.extract_topic(t)),
        };
        
        // Add agent to context
        self.context.agents.push(agent_ctx);
        
        // Render the LLM agent template
        debug!("Rendering LLM agent template");
        let template_content = self.template_manager.render_with_serializable(
            "src/agents/llm_agent.rs.tera", 
            &self.context
        )?;
        
        // Create the output file path
        let output_file = self.output_dir
            .join("src")
            .join("agents")
            .join(format!("{}.rs", agent_id.to_lowercase()));
            
        debug!(path = ?output_file.display(), "Creating directory structure for LLM agent");
        std::fs::create_dir_all(output_file.parent().unwrap())
            .context("Failed to create directory structure")?;
        
        debug!(path = %output_file.display(), "Writing generated LLM agent code");
        std::fs::write(&output_file, &template_content)
            .context("Failed to write LLM agent code")?;
            
        info!(agent_id = %agent_id, path = %output_file.display(), "Successfully generated LLM agent code");
        Ok(output_file)
    }
    
    /// Generate Rust code for a router agent
    #[instrument(skip(self, agent, workflow), fields(agent_id = ?agent.id, workflow_name = %workflow.name))]
    pub fn generate_router_agent(&mut self, agent: &Agent, workflow: &Workflow) -> Result<PathBuf> {
        info!("Generating Rust code for Router agent");
        let agent_id = agent.id.as_ref().ok_or_else(|| {
            error!("Agent is missing ID");
            TemplateError::Rendering(tera::Error::msg("Agent is missing ID".to_string()))
        })?;
        
        // Create agent context
        let agent_ctx = AgentContext {
            id: agent_id.clone(),
            agent_type: agent.agent_type.clone(),
            config: serde_json::to_value(agent.config.clone()).unwrap_or_default(),
            input_topic: workflow.source.as_ref().map(|s| self.extract_topic(s)),
            output_topic: workflow.target.as_ref().map(|t| self.extract_topic(t)),
        };
        
        // Add agent to context
        self.context.agents.push(agent_ctx);
        
        // Render the Router agent template
        debug!("Rendering Router agent template");
        let template_content = self.template_manager.render_with_serializable(
            "src/agents/router_agent.rs.tera", 
            &self.context
        )?;
        
        // Create the output file path
        let output_file = self.output_dir
            .join("src")
            .join("agents")
            .join(format!("{}.rs", agent_id.to_lowercase()));
        
        // Ensure the directory exists
        std::fs::create_dir_all(output_file.parent().unwrap())
            .context("Failed to create directory structure")?;
        
        // Write the generated code
        std::fs::write(&output_file, &template_content)
            .context("Failed to write Router agent code")?;
        
        info!(agent_id = %agent_id, path = %output_file.display(), "Successfully generated Router agent code");
        
        Ok(output_file)
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

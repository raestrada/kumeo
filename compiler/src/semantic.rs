use crate::ast::*;
use crate::error::{KumeoError, Result};
use std::collections::{HashMap, HashSet};
use tracing::{debug, error, info};

/// Semantic analyzer for Kumeo workflows
pub struct SemanticAnalyzer {
    /// Tracks defined agent IDs to detect duplicates
    agent_ids: HashSet<String>,
    /// Tracks defined workflow names
    workflow_names: HashSet<String>,
    /// Tracks defined source names
    source_names: HashSet<String>,
    /// Tracks defined target names
    target_names: HashSet<String>,
    /// Collection of detected errors
    errors: Vec<KumeoError>,
}

impl SemanticAnalyzer {
    /// Create a new semantic analyzer
    pub fn new() -> Self {
        Self {
            agent_ids: HashSet::new(),
            workflow_names: HashSet::new(),
            source_names: HashSet::new(),
            target_names: HashSet::new(),
            errors: Vec::new(),
        }
    }

    /// Analyze a workflow for semantic correctness
    pub fn analyze_workflow(&mut self, workflow: &Workflow) -> Result<()> {
        info!("Starting semantic analysis for workflow: {}", workflow.name);
        
        // Reset state for new analysis
        self.agent_ids.clear();
        self.source_names.clear();
        self.target_names.clear();
        self.errors.clear();

        // Validate workflow name
        self.validate_identifier(&workflow.name, "Workflow name")?;
        
        // Validate source if present
        if let Some(source) = &workflow.source {
            self.validate_source(source)?;
        } else {
            self.errors.push(KumeoError::SemanticError(
                "Workflow must have a source".to_string()
            ));
        }
        
        // Validate target if present
        if let Some(target) = &workflow.target {
            self.validate_target(target)?;
        } else {
            self.errors.push(KumeoError::SemanticError(
                "Workflow must have a target".to_string()
            ));
        }
        
        // Validate agents
        for agent in &workflow.agents {
            self.validate_agent(agent)?;
        }
        
        // If there are any errors, return the first one
        if let Some(first_error) = self.errors.first() {
            return Err(first_error.clone());
        }
        
        debug!("Semantic analysis completed successfully for workflow: {}", workflow.name);
        Ok(())
    }
    
    /// Validate a source definition
    fn validate_source(&mut self, source: &Source) -> Result<()> {
        debug!("Validating source: {:?}", source);
        
        match source {
            Source::NATS(topic, config) => {
                if topic.trim().is_empty() {
                    self.errors.push(KumeoError::SemanticError(
                        "NATS source must have a topic".to_string()
                    ));
                }
                
                if let Some(config) = config {
                    self.validate_config(config, "NATS source")?;
                }
            }
            // Add validation for other source types as needed
            _ => {}
        }
        
        Ok(())
    }
    
    /// Validate a target definition
    fn validate_target(&mut self, target: &Target) -> Result<()> {
        debug!("Validating target: {:?}", target);
        
        match target {
            Target::NATS(topic, config) => {
                if topic.trim().is_empty() {
                    self.errors.push(KumeoError::SemanticError(
                        "NATS target must have a topic".to_string()
                    ));
                }
                
                if let Some(config) = config {
                    self.validate_config(config, "NATS target")?;
                }
            }
            // Add validation for other target types as needed
            _ => {}
        }
        
        Ok(())
    }
    
    /// Validate an agent definition
    fn validate_agent(&mut self, agent: &Agent) -> Result<()> {
        debug!("Validating agent: {}", agent.id);
        
        // Check for duplicate agent IDs
        if !self.agent_ids.insert(agent.id.clone()) {
            self.errors.push(KumeoError::SemanticError(
                format!("Duplicate agent ID: {}", agent.id)
            ));
        }
        
        // Validate agent type
        match agent.agent_type {
            AgentType::LLM => self.validate_llm_agent(agent)?,
            AgentType::MLModel => self.validate_ml_agent(agent)?,
            _ => {}
        }
        
        // Validate configuration
        self.validate_config(&agent.config, &format!("Agent {}", agent.id))?;
        
        Ok(())
    }
    
    /// Validate an LLM agent
    fn validate_llm_agent(&self, agent: &Agent) -> Result<()> {
        // Check for required fields
        if !agent.config.contains_key("model") {
            self.errors.push(KumeoError::SemanticError(
                format!("LLM agent {} must specify a model", agent.id)
            ));
        }
        
        // Add more LLM-specific validation as needed
        
        Ok(())
    }
    
    /// Validate an ML model agent
    fn validate_ml_agent(&self, agent: &Agent) -> Result<()> {
        // Check for required fields
        if !agent.config.contains_key("model_path") {
            self.errors.push(KumeoError::SemanticError(
                format!("MLModel agent {} must specify a model_path", agent.id)
            ));
        }
        
        // Add more ML model-specific validation as needed
        
        Ok(())
    }
    
    /// Validate a configuration map
    fn validate_config(&self, config: &HashMap<String, Value>, context: &str) -> Result<()> {
        // Add any generic config validation here
        Ok(())
    }
    
    /// Validate an identifier (workflow name, agent ID, etc.)
    fn validate_identifier(&self, id: &str, context: &str) -> Result<()> {
        if id.trim().is_empty() {
            self.errors.push(KumeoError::SemanticError(
                format!("{} cannot be empty", context)
            ));
        }
        
        // Add more identifier validation as needed
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    fn create_test_workflow() -> Workflow {
        Workflow {
            name: "test_workflow".to_string(),
            description: None,
            nats: None,
            source: Some(Source::NATS("input.topic".to_string(), None)),
            target: Some(Target::NATS("output.topic".to_string(), None)),
            agents: vec![
                Agent {
                    id: "test_agent".to_string(),
                    agent_type: AgentType::LLM,
                    config: {
                        let mut map = HashMap::new();
                        map.insert("model".to_string(), Value::String("gpt-4".to_string()));
                        map
                    },
                }
            ],
        }
    }
    
    #[test]
    fn test_valid_workflow() {
        let mut analyzer = SemanticAnalyzer::new();
        let workflow = create_test_workflow();
        
        assert!(analyzer.analyze_workflow(&workflow).is_ok());
    }
    
    #[test]
    fn test_duplicate_agent_ids() {
        let mut analyzer = SemanticAnalyzer::new();
        let mut workflow = create_test_workflow();
        
        // Add a duplicate agent
        workflow.agents.push(Agent {
            id: "test_agent".to_string(),
            agent_type: AgentType::LLM,
            config: HashMap::new(),
        });
        
        let result = analyzer.analyze_workflow(&workflow);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            KumeoError::SemanticError(msg) if msg.contains("Duplicate agent ID")
        ));
    }
    
    #[test]
    fn test_missing_source() {
        let mut analyzer = SemanticAnalyzer::new();
        let mut workflow = create_test_workflow();
        workflow.source = None;
        
        let result = analyzer.analyze_workflow(&workflow);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            KumeoError::SemanticError(msg) if msg.contains("must have a source")
        ));
    }
    
    #[test]
    fn test_missing_target() {
        let mut analyzer = SemanticAnalyzer::new();
        let mut workflow = create_test_workflow();
        workflow.target = None;
        
        let result = analyzer.analyze_workflow(&workflow);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            KumeoError::SemanticError(msg) if msg.contains("must have a target")
        ));
    }
    
    #[test]
    fn test_llm_agent_without_model() {
        let mut analyzer = SemanticAnalyzer::new();
        let mut workflow = create_test_workflow();
        
        // Remove the model from the agent config
        if let Some(agent) = workflow.agents.first_mut() {
            agent.config.remove("model");
        }
        
        let result = analyzer.analyze_workflow(&workflow);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            KumeoError::SemanticError(msg) if msg.contains("must specify a model")
        ));
    }
}

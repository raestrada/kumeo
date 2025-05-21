use crate::ast::{Agent, AgentType, Argument, Integration, Program, Subworkflow, Workflow};
use crate::error::{KumeoError, Result};
use std::collections::{HashMap, HashSet};

/// Semantic analyzer for Kumeo programs
pub struct SemanticAnalyzer {
    /// Symbol table for storing declarations
    symbols: SymbolTable,
    /// Collection of detected errors
    errors: Vec<KumeoError>,
}

/// Symbol table for tracking declarations and references
pub struct SymbolTable {
    workflows: HashMap<String, Workflow>,
    subworkflows: HashMap<String, Subworkflow>,
}

impl SymbolTable {
    /// Create a new empty symbol table
    pub fn new() -> Self {
        Self {
            workflows: HashMap::new(),
            subworkflows: HashMap::new(),
        }
    }

    /// Add a workflow to the symbol table
    pub fn add_workflow(&mut self, workflow: &Workflow) -> Result<()> {
        if self.workflows.contains_key(&workflow.name) {
            return Err(KumeoError::SemanticError(format!("Duplicate workflow name: {}", workflow.name)));
        }
        self.workflows.insert(workflow.name.clone(), workflow.clone());
        Ok(())
    }

    /// Add a subworkflow to the symbol table
    pub fn add_subworkflow(&mut self, subworkflow: &Subworkflow) -> Result<()> {
        if self.subworkflows.contains_key(&subworkflow.name) {
            return Err(KumeoError::SemanticError(format!("Duplicate subworkflow name: {}", subworkflow.name)));
        }
        self.subworkflows.insert(subworkflow.name.clone(), subworkflow.clone());
        Ok(())
    }

    /// Check if a workflow exists
    pub fn workflow_exists(&self, name: &str) -> bool {
        self.workflows.contains_key(name)
    }

    /// Check if a subworkflow exists
    pub fn subworkflow_exists(&self, name: &str) -> bool {
        self.subworkflows.contains_key(name)
    }

    /// Get a workflow by name
    pub fn get_workflow(&self, name: &str) -> Option<&Workflow> {
        self.workflows.get(name)
    }

    /// Get a subworkflow by name
    pub fn get_subworkflow(&self, name: &str) -> Option<&Subworkflow> {
        self.subworkflows.get(name)
    }
}

impl SemanticAnalyzer {
    /// Create a new semantic analyzer
    pub fn new() -> Self {
        Self {
            symbols: SymbolTable::new(),
            errors: Vec::new(),
        }
    }

    /// Analyze a program for semantic correctness
    pub fn analyze(&mut self, program: &Program) -> Result<()> {
        // First pass: collect all declarations
        let _ = self.collect_declarations(program);
        
        // Second pass: validate references
        let _ = self.validate_references(program);
        
        // If there are any errors, return the first one
        if !self.errors.is_empty() {
            return Err(self.errors[0].clone());
        }
        
        Ok(())
    }

    /// First pass: collect all declarations
    fn collect_declarations(&mut self, program: &Program) -> Result<()> {
        // Collect all workflow declarations
        for workflow in &program.workflows {
            if let Err(e) = self.symbols.add_workflow(workflow) {
                self.errors.push(e);
            }
        }
        
        // Collect all subworkflow declarations
        for subworkflow in &program.subworkflows {
            if let Err(e) = self.symbols.add_subworkflow(subworkflow) {
                self.errors.push(e);
            }
        }
        
        Ok(())
    }

    /// Second pass: validate references
    fn validate_references(&mut self, program: &Program) -> Result<()> {
        // Validate integration references
        for integration in &program.integrations {
            self.validate_integration(integration)?;
        }
        
        // Validate agent references in workflows
        for workflow in &program.workflows {
            self.validate_workflow_agents(workflow)?;
            self.validate_workflow_source_target(workflow)?;
        }
        
        // Validate agent references in subworkflows
        for subworkflow in &program.subworkflows {
            self.validate_subworkflow_agents(subworkflow)?;
            self.validate_subworkflow_io(subworkflow)?;
        }
        
        Ok(())
    }
    
    /// Validate agent IDs within a workflow to ensure they're unique and present
    fn validate_workflow_agents(&mut self, workflow: &Workflow) -> Result<()> {
        let mut agent_ids = HashSet::new();
        
        // Check agents for unique IDs, that all agents have IDs, and proper configuration
        for (index, agent) in workflow.agents.iter().enumerate() {
            // Validate agent ID
            match &agent.id {
                Some(id) => {
                    if !agent_ids.insert(id.clone()) {
                        self.errors.push(KumeoError::SemanticError(
                            format!("Duplicate agent ID '{}' in workflow '{}'", id, workflow.name)
                        ));
                    }
                },
                None => {
                    self.errors.push(KumeoError::SemanticError(
                        format!("Agent at index {} in workflow '{}' is missing an ID. All agents must have an ID.", 
                                index, workflow.name)
                    ));
                }
            }
            
            // Validate agent configuration parameters
            let _ = self.validate_agent_configuration(agent, &workflow.name, false, index);
        }
        
        // Also check preprocessors if present
        if let Some(preprocessors) = &workflow.preprocessors {
            for (index, agent) in preprocessors.iter().enumerate() {
                // Validate agent ID
                match &agent.id {
                    Some(id) => {
                        if !agent_ids.insert(id.clone()) {
                            self.errors.push(KumeoError::SemanticError(
                                format!("Duplicate agent ID '{}' in workflow '{}' preprocessors", id, workflow.name)
                            ));
                        }
                    },
                    None => {
                        self.errors.push(KumeoError::SemanticError(
                            format!("Preprocessor agent at index {} in workflow '{}' is missing an ID. All agents must have an ID.", 
                                    index, workflow.name)
                        ));
                    }
                }
                
                // Validate agent configuration parameters
                let _ = self.validate_agent_configuration(agent, &workflow.name, true, index);
            }
        }
        
        Ok(())
    }
    
    /// Validate agent IDs within a subworkflow to ensure they're unique and present
    fn validate_subworkflow_agents(&mut self, subworkflow: &Subworkflow) -> Result<()> {
        let mut agent_ids = HashSet::new();
        
        // Check agents for unique IDs, that all agents have IDs, and proper configuration
        for (index, agent) in subworkflow.agents.iter().enumerate() {
            // Validate agent ID
            match &agent.id {
                Some(id) => {
                    if !agent_ids.insert(id.clone()) {
                        self.errors.push(KumeoError::SemanticError(
                            format!("Duplicate agent ID '{}' in subworkflow '{}'", id, subworkflow.name)
                        ));
                    }
                },
                None => {
                    self.errors.push(KumeoError::SemanticError(
                        format!("Agent at index {} in subworkflow '{}' is missing an ID. All agents must have an ID.", 
                                index, subworkflow.name)
                    ));
                }
            }
            
            // Validate agent configuration parameters
            // Pass false for is_preprocessor as subworkflows don't have a preprocessor concept
            let _ = self.validate_agent_configuration(agent, &subworkflow.name, false, index);
        }
        
        Ok(())
    }

    /// Validate that a workflow has properly configured source and target
    fn validate_workflow_source_target(&mut self, workflow: &Workflow) -> Result<()> {
        // Check that workflow has a source
        if workflow.source.is_none() {
            self.errors.push(KumeoError::SemanticError(
                format!("Workflow '{}' is missing a source configuration. Workflows require a source for event ingestion.", workflow.name)
            ));
        }

        // Check that workflow has a target
        if workflow.target.is_none() {
            self.errors.push(KumeoError::SemanticError(
                format!("Workflow '{}' is missing a target configuration. Workflows require a target for event propagation.", workflow.name)
            ));
        }

        Ok(())
    }
    
    /// Validate that a subworkflow has properly configured inputs and outputs
    fn validate_subworkflow_io(&mut self, subworkflow: &Subworkflow) -> Result<()> {
        // Check that subworkflow has defined inputs
        if subworkflow.input.is_none() || subworkflow.input.as_ref().unwrap().is_empty() {
            self.errors.push(KumeoError::SemanticError(
                format!("Subworkflow '{}' is missing input definitions. Subworkflows require defined inputs to receive data from workflows.", subworkflow.name)
            ));
        }
        
        // Check that subworkflow has defined outputs
        if subworkflow.output.is_none() || subworkflow.output.as_ref().unwrap().is_empty() {
            self.errors.push(KumeoError::SemanticError(
                format!("Subworkflow '{}' is missing output definitions. Subworkflows require defined outputs to return data to workflows.", subworkflow.name)
            ));
        }
        
        Ok(())
    }
    
    /// Validate an integration
    fn validate_integration(&mut self, integration: &Integration) -> Result<()> {
        // Check that referenced workflow exists
        if !self.symbols.workflow_exists(&integration.workflow) {
            self.errors.push(KumeoError::SemanticError(
                format!("Integration references undefined workflow: {}", integration.workflow)
            ));
        }
        
        // Check that referenced subworkflow exists
        if !self.symbols.subworkflow_exists(&integration.subworkflow) {
            self.errors.push(KumeoError::SemanticError(
                format!("Integration references undefined subworkflow: {}", integration.subworkflow)
            ));
        }
        
        // TODO: Validate input/output mappings match subworkflow's interface
        
        Ok(())
    }

    /// Validate agent configuration based on its type
    fn validate_agent_configuration(&mut self, agent: &Agent, workflow_name: &str, is_preprocessor: bool, index: usize) -> Result<()> {
        // Context string constructed but not used in current implementation
        // Kept for future use in error messages if needed
        let _agent_context = if is_preprocessor {
            format!("preprocessor agent at index {} in workflow '{}'", index, workflow_name)
        } else {
            format!("agent at index {} in workflow '{}'", index, workflow_name)
        };
        
        let agent_id = agent.id.as_ref().map_or("unknown".to_string(), |id| id.clone());
        
        // Get required parameters based on agent type
        let required_params = match agent.agent_type {
            AgentType::LLM => vec!["engine", "prompt"],
            AgentType::MLModel => vec!["model_path"],
            AgentType::BayesianNetwork => vec!["network_path"],
            AgentType::DecisionMatrix => vec!["matrix_definition"],
            AgentType::HumanInLoop => vec!["notification_channel"],
            AgentType::Router => vec!["routing_rules"],
            AgentType::Aggregator => vec!["aggregation_method"],
            AgentType::RuleEngine => vec!["rules"],
            AgentType::DataNormalizer => vec!["normalization_method"],
            AgentType::MissingValueHandler => vec!["handling_strategy"],
            AgentType::Custom(_) => vec![], // Custom agents can have variable params
        };
        
        // Convert agent configuration to a hashmap for easier lookup
        let mut config_map = HashMap::new();
        for arg in &agent.config {
            if let Argument::Named(name, _) = arg {
                config_map.insert(name.as_str(), arg);
            }
        }
        
        // Check that all required parameters are present
        for &param in &required_params {
            if !config_map.contains_key(param) {
                self.errors.push(KumeoError::SemanticError(
                    format!("Agent '{}' (type {:?}) is missing required parameter '{}'. This parameter is required for this agent type.", 
                            agent_id, agent.agent_type, param)
                ));
            }
        }
        
        Ok(())
    }

    /// Get all collected errors
    pub fn get_errors(&self) -> &Vec<KumeoError> {
        &self.errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Agent, AgentType, Argument, Value};

    #[test]
    fn test_duplicate_workflow_detection() {
        let mut analyzer = SemanticAnalyzer::new();
        
        // Create a workflow with properly configured source, target and an agent with all required parameters
        // to avoid other validation errors
        let workflow1 = Workflow {
            name: "test_workflow".to_string(),
            source: Some(crate::ast::Source::NATS("topic".to_string(), None)),
            target: Some(crate::ast::Target::NATS("topic".to_string(), None)),
            context: None,
            preprocessors: None,
            agents: vec![
                Agent {
                    id: Some("agent1".to_string()),
                    agent_type: AgentType::LLM,
                    config: vec![
                        Argument::Named("engine".to_string(), Value::String("gpt-4".to_string())),
                        Argument::Named("prompt".to_string(), Value::String("Hello".to_string())),
                    ],
                }
            ],
            monitor: None,
            deployment: None,
        };
        
        let workflow2 = workflow1.clone();
        
        let program = Program {
            workflows: vec![workflow1, workflow2],
            subworkflows: vec![],
            integrations: vec![],
        };
        
        let result = analyzer.analyze(&program);
        assert!(result.is_err());
        
        // Now we should only have one error for the duplicate workflow name
        let errors = analyzer.get_errors();
        let duplicate_workflow_errors = errors.iter()
            .filter(|e| {
                if let KumeoError::SemanticError(msg) = e {
                    msg.contains("Duplicate workflow name")
                } else {
                    false
                }
            })
            .count();
            
        assert_eq!(duplicate_workflow_errors, 1, "Expected exactly one duplicate workflow error");
        
        // Verify that we found a duplicate workflow error
        assert!(errors.iter().any(|e| {
            if let KumeoError::SemanticError(msg) = e {
                msg.contains("Duplicate workflow name")
            } else {
                false
            }
        }));
    }
}

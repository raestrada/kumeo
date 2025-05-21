use crate::ast::{Program, Workflow, Subworkflow, Agent, Integration, Context};
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
    // Track currently analyzed context for scope management
    current_context: Option<String>,
}

impl SymbolTable {
    /// Create a new empty symbol table
    pub fn new() -> Self {
        Self {
            workflows: HashMap::new(),
            subworkflows: HashMap::new(),
            current_context: None,
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
        self.collect_declarations(program);
        
        // Second pass: validate references
        self.validate_references(program);
        
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
        
        // TODO: Validate agent references, context, etc.
        
        Ok(())
    }

    /// Validate an integration
    fn validate_integration(&mut self, integration: &Integration) -> Result<()> {
        // Check if the referenced workflow exists
        if !self.symbols.workflow_exists(&integration.workflow) {
            self.errors.push(KumeoError::SemanticError(
                format!("Integration references undefined workflow: {}", integration.workflow)
            ));
        }
        
        // Check if the referenced subworkflow exists
        if !self.symbols.subworkflow_exists(&integration.subworkflow) {
            self.errors.push(KumeoError::SemanticError(
                format!("Integration references undefined subworkflow: {}", integration.subworkflow)
            ));
        }
        
        // TODO: Validate input/output mappings match subworkflow's interface
        
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
        
        let workflow1 = Workflow {
            name: "test_workflow".to_string(),
            source: None,
            target: None,
            context: None,
            preprocessors: None,
            agents: vec![],
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
        assert_eq!(analyzer.errors.len(), 1);
        match &analyzer.errors[0] {
            KumeoError::SemanticError(msg) => {
                assert!(msg.contains("Duplicate workflow name"));
            },
            _ => panic!("Expected SemanticError"),
        }
    }
}

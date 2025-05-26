//! Implementación del analizador semántico para Kumeo.

use std::collections::{HashMap, HashSet};

use crate::{
    ast::*,
    error::{KumeoError, Result},
};

/// Analizador semántico para programas Kumeo.
#[derive(Debug)]
pub struct SemanticAnalyzer {
    /// IDs de agentes definidos (para detectar duplicados)
    agent_ids: HashSet<String>,
    /// Nombres de workflows definidos
    workflow_names: HashSet<String>,
    /// Nombres de subworkflows definidos
    subworkflow_names: HashSet<String>,
    /// Errores encontrados durante el análisis
    errors: Vec<KumeoError>,
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl SemanticAnalyzer {
    /// Crea un nuevo analizador semántico.
    pub fn new() -> Self {
        Self {
            agent_ids: HashSet::new(),
            workflow_names: HashSet::new(),
            subworkflow_names: HashSet::new(),
            errors: Vec::new(),
        }
    }

    /// Realiza el análisis semántico de un programa completo.
    pub fn analyze_program(&mut self, program: &Program) -> Result<()> {
        self.reset();

        // Validar nombres únicos de workflows y subworkflows
        let mut all_names = HashSet::new();
        
        for workflow in &program.workflows {
            if !all_names.insert(&workflow.name) {
                self.errors.push(KumeoError::SemanticError(
                    format!("Nombre de workflow duplicado: {}", workflow.name),
                ));
            }
            self.workflow_names.insert(workflow.name.clone());
        }

        for subworkflow in &program.subworkflows {
            if !all_names.insert(&subworkflow.name) {
                self.errors.push(KumeoError::SemanticError(
                    format!("Nombre de subworkflow duplicado: {}", subworkflow.name),
                ));
            }
            self.subworkflow_names.insert(subworkflow.name.clone());
        }

        // Validar cada workflow
        for workflow in &program.workflows {
            self.analyze_workflow(workflow)?;
        }

        // Validar cada subworkflow
        for subworkflow in &program.subworkflows {
            self.analyze_subworkflow(subworkflow)?;
        }

        // Crear una copia de los errores para no mover self
        let errors = self.errors.clone();
        
        if errors.is_empty() {
            Ok(())
        } else {
            let error_messages: Vec<String> = errors.iter()
                .map(|e| e.to_string())
                .collect();
            Err(KumeoError::SemanticErrors(error_messages))
        }
    }

    /// Analiza un workflow individual.
    pub fn analyze_workflow(&mut self, workflow: &Workflow) -> Result<()> {
        // Validar nombre
        self.validate_identifier(&workflow.name, "workflow")?;

        // Validar fuente
        if let Some(source) = &workflow.source {
            self.validate_source(source)?;
        } else {
            self.errors.push(KumeoError::SemanticError(
                "El workflow debe tener una fuente de datos".to_string(),
            ));
        }

        // Validar destino
        if let Some(target) = &workflow.target {
            self.validate_target(target)?;
        }

        // Validar agentes
        for agent in &workflow.agents {
            self.validate_agent(agent)?;
        }

        // Validar preprocesadores
        if let Some(preprocessors) = &workflow.preprocessors {
            for preprocessor in preprocessors {
                self.validate_agent(preprocessor)?;
            }
        }

        Ok(())
    }

    /// Analiza un subworkflow individual.
    pub fn analyze_subworkflow(&mut self, subworkflow: &Subworkflow) -> Result<()> {
        // Validar nombre
        self.validate_identifier(&subworkflow.name, "subworkflow")?;

        // Validar agentes
        for agent in &subworkflow.agents {
            self.validate_agent(agent)?;
        }

        Ok(())
    }

    /// Valida una fuente de datos.
    fn validate_source(&mut self, source: &Source) -> Result<()> {
        match source {
            Source::NATS(topic, _) => {
                if topic.trim().is_empty() {
                    self.errors.push(KumeoError::SemanticError(
                        "El tema de NATS no puede estar vacío".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }

    /// Valida un destino de datos.
    fn validate_target(&mut self, target: &Target) -> Result<()> {
        match target {
            Target::NATS(topic, _) => {
                if topic.trim().is_empty() {
                    self.errors.push(KumeoError::SemanticError(
                        "El tema de NATS no puede estar vacío".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }

    /// Valida un agente.
    fn validate_agent(&mut self, agent: &Agent) -> Result<()> {
        // Validar ID único
        if let Some(id) = &agent.id {
            if !self.agent_ids.insert(id.clone()) {
                self.errors.push(KumeoError::SemanticError(
                    format!("ID de agente duplicado: {}", id),
                ));
            }
        } else {
            self.errors.push(KumeoError::SemanticError(
                "Todos los agentes deben tener un ID".to_string(),
            ));
        }

        // Validar configuración específica del tipo de agente
        match agent.agent_type {
            AgentType::LLM => self.validate_llm_agent(agent)?,
            AgentType::MLModel => self.validate_ml_agent(agent)?,
            _ => {}
        }

        Ok(())
    }

    /// Valida un agente LLM.
    fn validate_llm_agent(&self, agent: &Agent) -> Result<()> {
        // Verificar que tenga el campo 'model' configurado
        let has_model = agent.config.iter().any(|arg| match arg {
            Argument::Named(name, _) => name == "model",
            _ => false,
        });

        if !has_model {
            return Err(KumeoError::SemanticError(
                "Los agentes LLM deben tener un modelo configurado".to_string(),
            ));
        }

        Ok(())
    }

    /// Valida un agente de modelo de ML.
    fn validate_ml_agent(&self, agent: &Agent) -> Result<()> {
        // Verificar que tenga el campo 'model_path' o 'model_name' configurado
        let has_model = agent.config.iter().any(|arg| match arg {
            Argument::Named(name, _) => name == "model_path" || name == "model_name",
            _ => false,
        });

        if !has_model {
            return Err(KumeoError::SemanticError(
                "Los agentes de ML deben tener 'model_path' o 'model_name' configurado".to_string(),
            ));
        }

        Ok(())
    }

    /// Valida un identificador (nombre de workflow, subworkflow, etc.).
    fn validate_identifier(&self, id: &str, context: &str) -> Result<()> {
        if id.trim().is_empty() {
            return Err(KumeoError::SemanticError(
                format!("El {} no puede tener un nombre vacío", context),
            ));
        }

        // Validar que solo contenga caracteres alfanuméricos y guiones bajos
        if !id.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Err(KumeoError::SemanticError(format!(
                "El {} '{}' solo puede contener caracteres alfanuméricos y guiones bajos",
                context, id
            )));
        }

        // Validar que no empiece con un número
        if let Some(first_char) = id.chars().next() {
            if first_char.is_ascii_digit() {
                return Err(KumeoError::SemanticError(format!(
                    "El {} '{}' no puede empezar con un número",
                    context, id
                )));
            }
        }

        Ok(())
    }

    /// Reinicia el estado del analizador.
    fn reset(&mut self) {
        self.agent_ids.clear();
        self.workflow_names.clear();
        self.subworkflow_names.clear();
        self.errors.clear();
    }
}

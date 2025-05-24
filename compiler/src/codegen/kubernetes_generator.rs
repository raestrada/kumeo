use std::collections::HashMap;
use std::path::{Path, PathBuf};
use crate::ast::{Program, Workflow, Subworkflow, Agent};
use crate::codegen::template_manager::{TemplateManager, Result, TemplateError};
use tracing::{debug, info, warn, error, instrument};

pub struct KubernetesGenerator {
    template_manager: TemplateManager,
    output_dir: PathBuf,
}

impl KubernetesGenerator {
    pub fn new<P: AsRef<Path>>(template_root: P, output_dir: P) -> Result<Self> {
        debug!(template_path = ?template_root.as_ref().display(), "Initializing Kubernetes generator");
        let template_manager = TemplateManager::new(template_root.as_ref().join("kubernetes"))?;
        let output_dir = output_dir.as_ref().join("kubernetes");
        
        // Create output directory if it doesn't exist
        if !output_dir.exists() {
            debug!(path = ?output_dir.display(), "Creating Kubernetes output directory");
            std::fs::create_dir_all(&output_dir)?;
        }
        
        info!("Kubernetes generator initialized");
        
        Ok(Self {
            template_manager,
            output_dir,
        })
    }
    
    /// Generate Kubernetes manifests for the entire program
    pub fn generate_kubernetes_manifests(&mut self, program: &Program) -> Result<PathBuf> {
        info!("Generating Kubernetes manifests for program");
        debug!(workflows_count = program.workflows.len(), "Program details");
        
        // Generate a namespace for the program
        self.generate_namespace(program)?;
        
        // Generate NATS infrastructure
        self.generate_nats_infrastructure()?;
        
        // Generate manifests for each workflow
        for workflow in &program.workflows {
            self.generate_workflow_manifests(workflow)?;
        }
        
        // Generate manifests for each subworkflow
        for subworkflow in &program.subworkflows {
            self.generate_subworkflow_manifests(subworkflow)?;
        }
        
        // Generate the combined kustomization file
        self.generate_kustomization(program)?;
        
        Ok(self.output_dir.clone())
    }
    
    /// Generate a Kubernetes namespace for the program
    #[instrument(skip(self, _program))]
    pub fn generate_namespace(&mut self, _program: &Program) -> Result<PathBuf> {
        info!("Generating Kubernetes namespace manifest");
        let mut params = HashMap::new();
        params.insert("namespace".to_string(), "kumeo".to_string());
        
        debug!("Rendering namespace template");
        let template_content = self.template_manager.render_template("namespace.yaml", &params)?;
        
        let output_file = self.output_dir.join("namespace.yaml");
        debug!(path = ?output_file.display(), "Writing namespace manifest");
        
        match std::fs::write(&output_file, &template_content) {
            Ok(_) => {
                info!(path = %output_file.display(), "Successfully generated namespace manifest");
                Ok(output_file)
            },
            Err(e) => {
                error!(error = %e, path = %output_file.display(), "Failed to write namespace manifest");
                Err(TemplateError::Io(e))
            }
        }
    }
    
    /// Generate NATS infrastructure manifests
    #[instrument(skip(self))]
    pub fn generate_nats_infrastructure(&mut self) -> Result<PathBuf> {
        info!("Generating NATS infrastructure manifests");
        let params = HashMap::new();
        
        debug!("Rendering NATS infrastructure template");
        let template_content = self.template_manager.render_template("nats.yaml", &params)?;
        
        let output_file = self.output_dir.join("nats.yaml");
        debug!(path = ?output_file.display(), "Writing NATS infrastructure manifest");
        
        match std::fs::write(&output_file, &template_content) {
            Ok(_) => {
                info!(path = %output_file.display(), "Successfully generated NATS infrastructure manifest");
                Ok(output_file)
            },
            Err(e) => {
                error!(error = %e, path = %output_file.display(), "Failed to write NATS infrastructure manifest");
                Err(TemplateError::Io(e))
            }
        }
    }
    
    /// Generate Kubernetes manifests for a workflow
    fn generate_workflow_manifests(&mut self, workflow: &Workflow) -> Result<PathBuf> {
        println!("Generating manifests for workflow: {}", workflow.name);
        
        // Create a directory for the workflow
        let workflow_dir = self.output_dir.join(&workflow.name);
        std::fs::create_dir_all(&workflow_dir)?;
        
        // Generate manifests for each agent in the workflow
        for (index, agent) in workflow.agents.iter().enumerate() {
            self.generate_agent_manifests(agent, workflow, &workflow_dir, index)?;
        }
        
        // Generate manifests for preprocessors if present
        if let Some(preprocessors) = &workflow.preprocessors {
            for (index, agent) in preprocessors.iter().enumerate() {
                self.generate_preprocessor_manifests(agent, workflow, &workflow_dir, index)?;
            }
        }
        
        // Generate ConfigMaps for workflow context if present
        // Generate ConfigMaps for workflow context
        self.generate_context_configmaps(&workflow.context, workflow, &workflow_dir)?;
        
        // Generate a kustomization file for the workflow
        self.generate_workflow_kustomization(workflow, &workflow_dir)?;
        
        Ok(workflow_dir)
    }
    
    /// Generate Kubernetes manifests for a subworkflow
    fn generate_subworkflow_manifests(&mut self, subworkflow: &Subworkflow) -> Result<PathBuf> {
        println!("Generating manifests for subworkflow: {}", subworkflow.name);
        
        // Create a directory for the subworkflow
        let subworkflow_dir = self.output_dir.join(&subworkflow.name);
        std::fs::create_dir_all(&subworkflow_dir)?;
        
        // Generate manifests for each agent in the subworkflow
        for (index, agent) in subworkflow.agents.iter().enumerate() {
            // For subworkflows, we use a temporary workflow structure
            let temp_workflow = Workflow {
                name: subworkflow.name.clone(),
                source: None,
                target: None,
                context: subworkflow.context.clone(),
                preprocessors: None,
                agents: vec![],
                monitor: None,
                deployment: None,
            };
            
            self.generate_agent_manifests(agent, &temp_workflow, &subworkflow_dir, index)?;
        }
        
        // Generate ConfigMaps for subworkflow context
        let temp_workflow = Workflow {
            name: subworkflow.name.clone(),
            source: None,
            target: None,
            context: subworkflow.context.clone(),
            preprocessors: None,
            agents: vec![],
            monitor: None,
            deployment: None,
        };
        self.generate_context_configmaps(&subworkflow.context, &temp_workflow, &subworkflow_dir)?;
        
        // Generate a kustomization file for the subworkflow
        self.generate_subworkflow_kustomization(subworkflow, &subworkflow_dir)?;
        
        Ok(subworkflow_dir)
    }
    
    /// Generate Kubernetes manifests for an agent
    fn generate_agent_manifests(&mut self, agent: &Agent, workflow: &Workflow, output_dir: &Path, index: usize) -> Result<PathBuf> {
        let agent_id = agent.id.as_ref().ok_or_else(|| {
            TemplateError::Rendering("Agent is missing ID".to_string())
        })?;
        
        // Determine which language the agent uses based on its type
        let language = match agent.agent_type {
            crate::ast::AgentType::LLM | 
            crate::ast::AgentType::Router | 
            crate::ast::AgentType::Aggregator | 
            crate::ast::AgentType::RuleEngine | 
            crate::ast::AgentType::DataNormalizer | 
            crate::ast::AgentType::MissingValueHandler => "rust",
            
            crate::ast::AgentType::MLModel | 
            crate::ast::AgentType::BayesianNetwork | 
            crate::ast::AgentType::DecisionMatrix => "python",
            
            crate::ast::AgentType::HumanInLoop => "hybrid",
            crate::ast::AgentType::Custom(_) => "rust", // Default to rust for custom agents
        };
        
        // Extract agent configuration
        let mut params = HashMap::new();
        params.insert("agent_id".to_string(), agent_id.clone());
        params.insert("workflow_name".to_string(), workflow.name.clone());
        params.insert("language".to_string(), language.to_string());
        params.insert("index".to_string(), index.to_string());
        
        // Extract agent type
        params.insert("agent_type".to_string(), format!("{:?}", agent.agent_type));
        
        // Render the deployment template
        let deployment = self.template_manager.render_template("deployment.yaml.tmpl", &params)?;
        
        // Write the deployment manifest to a file
        let deployment_file = output_dir.join(format!("{}-deployment.yaml", agent_id.to_lowercase()));
        std::fs::write(&deployment_file, deployment)?;
        
        // Render the service template if needed
        let service = self.template_manager.render_template("service.yaml.tmpl", &params)?;
        
        // Write the service manifest to a file
        let service_file = output_dir.join(format!("{}-service.yaml", agent_id.to_lowercase()));
        std::fs::write(&service_file, service)?;
        
        // Generate ConfigMap for agent configuration
        self.generate_agent_configmap(agent, workflow, output_dir)?;
        
        Ok(deployment_file)
    }
    
    /// Generate Kubernetes manifests for a preprocessor agent
    fn generate_preprocessor_manifests(&mut self, agent: &Agent, workflow: &Workflow, output_dir: &Path, index: usize) -> Result<PathBuf> {
        // Similar to generate_agent_manifests but with specific handling for preprocessors
        self.generate_agent_manifests(agent, workflow, output_dir, index)
    }
    
    /// Generate ConfigMap for agent configuration
    fn generate_agent_configmap(&mut self, agent: &Agent, workflow: &Workflow, output_dir: &Path) -> Result<PathBuf> {
        let agent_id = agent.id.as_ref().ok_or_else(|| {
            TemplateError::Rendering("Agent is missing ID".to_string())
        })?;
        
        // Extract agent configuration
        let mut params = HashMap::new();
        params.insert("agent_id".to_string(), agent_id.clone());
        params.insert("workflow_name".to_string(), workflow.name.clone());
        
        // Add agent config as JSON
        let config_json = serde_json::to_string_pretty(&agent.config)
            .unwrap_or_else(|_| "{}".to_string());
        params.insert("config_json".to_string(), config_json);
        
        // Render the configmap template
        let configmap = self.template_manager.render_template("configmap.yaml.tmpl", &params)?;
        
        // Write the configmap manifest to a file
        let configmap_file = output_dir.join(format!("{}-configmap.yaml", agent_id.to_lowercase()));
        std::fs::write(&configmap_file, configmap)?;
        
        Ok(configmap_file)
    }
    
    /// Generate ConfigMaps for workflow context
    fn generate_context_configmaps(&mut self, context: &Option<crate::ast::Context>, workflow: &Workflow, output_dir: &Path) -> Result<PathBuf> {
        // Create a configmap for the context items
        let mut params = HashMap::new();
        params.insert("workflow_name".to_string(), workflow.name.clone());
        
        // Add context as JSON if it exists
        let context_json = match context {
            Some(ctx) => serde_json::to_string_pretty(ctx)
                .unwrap_or_else(|_| "{}".to_string()),
            None => "{}".to_string()
        };
        params.insert("context_json".to_string(), context_json);
        
        // Render the context configmap template
        let configmap = self.template_manager.render_template("context-configmap.yaml.tmpl", &params)?;
        
        // Write the configmap manifest to a file
        let configmap_file = output_dir.join("context-configmap.yaml");
        std::fs::write(&configmap_file, configmap)?;
        
        Ok(configmap_file)
    }
    
    /// Generate kustomization file for a workflow
    fn generate_workflow_kustomization(&mut self, workflow: &Workflow, output_dir: &Path) -> Result<PathBuf> {
        let mut params = HashMap::new();
        params.insert("workflow_name".to_string(), workflow.name.clone());
        
        // Get a list of all manifest files in the workflow directory
        let files = std::fs::read_dir(output_dir)?
            .filter_map(|entry| {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().map_or(false, |ext| ext == "yaml") {
                        return Some(path.file_name().unwrap().to_string_lossy().to_string());
                    }
                }
                None
            })
            .collect::<Vec<String>>()
            .join("\n  - ");
        
        params.insert("resource_files".to_string(), files);
        
        // Render the kustomization template
        let kustomization = self.template_manager.render_template("kustomization.yaml.tmpl", &params)?;
        
        // Write the kustomization file
        let kustomization_file = output_dir.join("kustomization.yaml");
        std::fs::write(&kustomization_file, kustomization)?;
        
        Ok(kustomization_file)
    }
    
    /// Generate kustomization file for a subworkflow
    fn generate_subworkflow_kustomization(&mut self, subworkflow: &Subworkflow, output_dir: &Path) -> Result<PathBuf> {
        // Similar to generate_workflow_kustomization but for subworkflows
        let mut params = HashMap::new();
        params.insert("workflow_name".to_string(), subworkflow.name.clone());
        
        // Get a list of all manifest files in the subworkflow directory
        let files = std::fs::read_dir(output_dir)?
            .filter_map(|entry| {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().map_or(false, |ext| ext == "yaml") {
                        return Some(path.file_name().unwrap().to_string_lossy().to_string());
                    }
                }
                None
            })
            .collect::<Vec<String>>()
            .join("\n  - ");
        
        params.insert("resource_files".to_string(), files);
        
        // Render the kustomization template
        let kustomization = self.template_manager.render_template("kustomization.yaml.tmpl", &params)?;
        
        // Write the kustomization file
        let kustomization_file = output_dir.join("kustomization.yaml");
        std::fs::write(&kustomization_file, kustomization)?;
        
        Ok(kustomization_file)
    }
    
    /// Generate the root kustomization file for the entire program
    fn generate_kustomization(&mut self, _program: &Program) -> Result<PathBuf> {
        let mut params = HashMap::new();
        
        // Get a list of all directories (workflows and subworkflows)
        let dirs = std::fs::read_dir(&self.output_dir)?
            .filter_map(|entry| {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        return Some(path.file_name().unwrap().to_string_lossy().to_string());
                    }
                }
                None
            })
            .collect::<Vec<String>>();
        
        // Add base resources (namespace, NATS)
        let mut resources = vec![
            "namespace.yaml".to_string(),
            "nats.yaml".to_string(),
        ];
        
        // Add workflow and subworkflow references
        for dir in dirs {
            resources.push(format!("./{}", dir));
        }
        
        params.insert("resource_files".to_string(), resources.join("\n  - "));
        
        // Render the kustomization template
        let kustomization = self.template_manager.render_template("root-kustomization.yaml.tmpl", &params)?;
        
        // Write the kustomization file
        let kustomization_file = self.output_dir.join("kustomization.yaml");
        std::fs::write(&kustomization_file, kustomization)?;
        
        Ok(kustomization_file)
    }
}

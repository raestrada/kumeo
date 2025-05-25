use std::collections::HashMap;
use std::path::{Path, PathBuf};
use serde::Serialize;
use anyhow::Context;
use crate::ast::{Program, Workflow, Subworkflow, Agent, AgentType};
use crate::codegen::template_manager::{TemplateManager, Result, TemplateError};
use tracing::{debug, info, warn, error, instrument};

/// Context for Kubernetes manifest generation
#[derive(Debug, Serialize)]
struct KubernetesContext {
    project_name: String,
    namespace: String,
    version: String,
    workflows: Vec<WorkflowContext>,
    subworkflows: Vec<SubworkflowContext>,
}

/// Context for workflow manifest generation
#[derive(Debug, Serialize)]
struct WorkflowContext {
    name: String,
    namespace: String,
    agents: Vec<AgentContext>,
}

/// Context for subworkflow manifest generation
#[derive(Debug, Serialize)]
struct SubworkflowContext {
    name: String,
    namespace: String,
    agents: Vec<AgentContext>,
}

/// Context for agent manifest generation
#[derive(Debug, Serialize)]
struct AgentContext {
    name: String,
    agent_type: AgentType,
    namespace: String,
    image: String,
    config: serde_json::Value,
    input_topic: Option<String>,
    output_topic: Option<String>,
    resources: ResourceRequirements,
}

/// Resource requirements for Kubernetes containers
#[derive(Debug, Serialize)]
struct ResourceRequirements {
    requests: ResourceQuantity,
    limits: ResourceQuantity,
}

/// Resource quantity for Kubernetes
#[derive(Debug, Serialize)]
struct ResourceQuantity {
    cpu: String,
    memory: String,
}

pub struct KubernetesGenerator {
    template_manager: TemplateManager,
    output_dir: PathBuf,
    context: KubernetesContext,
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
        
        // Initialize context with default values
        let context = KubernetesContext {
            project_name: "kumeo".to_string(),
            namespace: "kumeo".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            workflows: Vec::new(),
            subworkflows: Vec::new(),
        };
        
        info!("Kubernetes generator initialized");
        
        Ok(Self {
            template_manager,
            output_dir,
            context,
        })
    }
    
    /// Generate Kubernetes manifests for the entire program
    pub fn generate_kubernetes_manifests(&mut self, program: &Program) -> Result<PathBuf> {
        info!("Generating Kubernetes manifests for program");
        debug!(workflows_count = program.workflows.len(), "Program details");
        
        // Update context with program details
        self.context.project_name = program.name.clone().unwrap_or_else(|| "kumeo".to_string());
        
        // Generate a namespace for the program
        self.generate_namespace(program)?;
        
        // Generate NATS infrastructure
        self.generate_nats_infrastructure()?;
        
        // Generate manifests for each workflow
        for workflow in &program.workflows {
            self.context.workflows.push(WorkflowContext {
                name: workflow.name.clone(),
                namespace: self.context.namespace.clone(),
                agents: Vec::new(),
            });
            self.generate_workflow_manifests(workflow)?;
        }
        
        // Generate manifests for each subworkflow
        for subworkflow in &program.subworkflows {
            self.context.subworkflows.push(SubworkflowContext {
                name: subworkflow.name.clone(),
                namespace: self.context.namespace.clone(),
                agents: Vec::new(),
            });
            self.generate_subworkflow_manifests(subworkflow)?;
        }
        
        // Generate the root kustomization file
        self.generate_kustomization(program)?;
        
        info!(
            "Successfully generated all Kubernetes manifests in {}", 
            self.output_dir.display()
        );
        
        Ok(self.output_dir.clone())
    }
    
    /// Generate a Kubernetes namespace for the program
    fn generate_namespace(&mut self, _program: &Program) -> Result<PathBuf> {
        info!("Generating Kubernetes namespace");
        
        // Create context for the template
        let namespace_ctx = serde_json::json!({
            "namespace": self.context.namespace,
            "project_name": self.context.project_name,
        });
        
        // Create the namespace directory
        let namespace_dir = self.output_dir.join("namespace");
        std::fs::create_dir_all(&namespace_dir)
            .context("Failed to create namespace directory")?;
        
        // Render the namespace template
        let namespace = self.template_manager.render_with_serializable(
            "namespace/namespace.yaml.tera",
            &namespace_ctx
        )?;
        
        // Write the namespace manifest to a file
        let namespace_file = namespace_dir.join("namespace.yaml");
        std::fs::write(&namespace_file, namespace)
            .context("Failed to write namespace manifest")?;
        
        // Generate kustomization.yaml for the namespace
        let kustomization = self.template_manager.render_with_serializable(
            "namespace/kustomization.yaml.tera",
            &namespace_ctx
        )?;
        std::fs::write(namespace_dir.join("kustomization.yaml"), kustomization)
            .context("Failed to write namespace kustomization")?;
        
        info!(
            namespace = %self.context.namespace,
            path = %namespace_file.display(),
            "Successfully generated Kubernetes namespace"
        );
        
        Ok(namespace_file)
    }
    
    /// Generate NATS infrastructure manifests
    #[instrument(skip(self))]
    pub fn generate_nats_infrastructure(&mut self) -> Result<PathBuf> {
        info!("Generating NATS infrastructure manifests");
        
        // Create context for the template
        let nats_ctx = serde_json::json!({
            "namespace": self.context.namespace,
            "project_name": self.context.project_name,
        });
        
        // Create the NATS directory
        let nats_dir = self.output_dir.join("nats");
        std::fs::create_dir_all(&nats_dir)
            .context("Failed to create NATS directory")?;
        
        // Render the NATS StatefulSet template
        let statefulset = self.template_manager.render_with_serializable(
            "nats/statefulset.yaml.tera",
            &nats_ctx
        )?;
        
        // Write the StatefulSet manifest to a file
        let statefulset_file = nats_dir.join("statefulset.yaml");
        std::fs::write(&statefulset_file, statefulset)
            .context("Failed to write NATS StatefulSet manifest")?;
        
        // Render the NATS Service template
        let service = self.template_manager.render_with_serializable(
            "nats/service.yaml.tera",
            &nats_ctx
        )?;
        
        // Write the Service manifest to a file
        let service_file = nats_dir.join("service.yaml");
        std::fs::write(&service_file, service)
            .context("Failed to write NATS Service manifest")?;
        
        // Generate kustomization.yaml for NATS
        let kustomization = self.template_manager.render_with_serializable(
            "nats/kustomization.yaml.tera",
            &nats_ctx
        )?;
        std::fs::write(nats_dir.join("kustomization.yaml"), kustomization)
            .context("Failed to write NATS kustomization")?;
        
        info!(
            namespace = %self.context.namespace,
            path = %nats_dir.display(),
            "Successfully generated NATS infrastructure"
        );
        
        Ok(nats_dir)
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
            error!("Agent is missing ID");
            TemplateError::Rendering("Agent is missing ID".to_string())
        })?;
        
        debug!("Generating manifests for agent: {}", agent_id);
        
        // Create agent context
        let agent_ctx = AgentContext {
            name: agent_id.clone(),
            agent_type: agent.agent_type.clone(),
            namespace: self.context.namespace.clone(),
            image: format!("kumeo/{}:latest", agent.agent_type.to_string().to_lowercase()),
            config: serde_json::to_value(agent.config.clone()).unwrap_or_default(),
            input_topic: workflow.source.as_ref().map(|s| format!("{}", s.topic)),
            output_topic: workflow.target.as_ref().map(|t| format!("{}", t.topic)),
            resources: ResourceRequirements {
                requests: ResourceQuantity {
                    cpu: "100m".to_string(),
                    memory: "128Mi".to_string(),
                },
                limits: ResourceQuantity {
                    cpu: "500m".to_string(),
                    memory: "512Mi".to_string(),
                },
            },
        };
        
        // Add agent to the appropriate context (workflow or subworkflow)
        if let Some(workflow_ctx) = self.context.workflows.iter_mut()
            .find(|w| w.name == workflow.name) {
            workflow_ctx.agents.push(agent_ctx.clone());
        } else if let Some(subworkflow_ctx) = self.context.subworkflows.iter_mut()
            .find(|sw| sw.name == workflow.name) {
            subworkflow_ctx.agents.push(agent_ctx.clone());
        }
        
        // Create agent directory
        let agent_dir = output_dir.join(agent_id);
        std::fs::create_dir_all(&agent_dir)
            .context("Failed to create agent directory")?;
        
        // Generate ConfigMap for agent configuration
        self.generate_agent_configmap(agent, workflow, &agent_dir)?;
        
        // Generate Deployment using template
        let deployment_path = agent_dir.join("deployment.yaml");
        let deployment = self.template_manager.render_with_serializable(
            "agent/deployment.yaml.tera",
            &agent_ctx
        )?;
        std::fs::write(&deployment_path, deployment)
            .context("Failed to write deployment manifest")?;
        
        // Generate Service using template
        let service_path = agent_dir.join("service.yaml");
        let service = self.template_manager.render_with_serializable(
            "agent/service.yaml.tera",
            &agent_ctx
        )?;
        std::fs::write(&service_path, service)
            .context("Failed to write service manifest")?;
        
        // Generate kustomization.yaml
        let kustomization_path = agent_dir.join("kustomization.yaml");
        let kustomization = self.template_manager.render_with_serializable(
            "kustomization.yaml.tera",
            &self.context
        )?;
        std::fs::write(&kustomization_path, kustomization)
            .context("Failed to write kustomization")?;
        
        info!(
            agent_id = %agent_id, 
            path = %agent_dir.display(), 
            "Successfully generated Kubernetes manifests for agent"
        );
        
        Ok(agent_dir)
    }
    
    /// Generate ConfigMaps for workflow context
    fn generate_context_configmaps(
        &mut self, 
        context: &Option<crate::ast::Context>, 
        workflow: &Workflow, 
        output_dir: &Path
    ) -> Result<PathBuf> {
        debug!("Generating ConfigMaps for workflow context: {}", workflow.name);
        
        // Create context for the template
        let context_ctx = serde_json::json!({
            "workflow_name": workflow.name,
            "namespace": self.context.namespace,
            "context": context.as_ref().map(|c| serde_json::to_value(c)).unwrap_or_else(|| serde_json::Value::Object(serde_json::Map::new()))
        });
        
        // Render the context configmap template
        let configmap = self.template_manager.render_with_serializable(
            "context/configmap.yaml.tera",
            &context_ctx
        )?;
        
        // Write the configmap manifest to a file
        let configmap_file = output_dir.join("context-configmap.yaml");
        std::fs::write(&configmap_file, configmap)
            .context("Failed to write context ConfigMap")?;
        
        debug!(
            workflow = %workflow.name,
            path = %configmap_file.display(),
            "Successfully generated context ConfigMap"
        );
        
        Ok(configmap_file)
    }
    
    /// Generate kustomization file for a workflow
    fn generate_workflow_kustomization(&mut self, workflow: &Workflow, output_dir: &Path) -> Result<PathBuf> {
        debug!("Generating kustomization for workflow: {}", workflow.name);
        
        // Get a list of all YAML files in the workflow directory
        let resource_files = std::fs::read_dir(output_dir)?
            .filter_map(|entry| {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().map_or(false, |ext| ext == "yaml") {
                        return Some(path.file_name()?.to_string_lossy().to_string());
                    }
                }
                None
            })
            .collect::<Vec<_>>();
        
        // Create context for the template
        let kustomization_ctx = serde_json::json!({
            "workflow_name": workflow.name,
            "namespace": self.context.namespace,
            "resource_files": resource_files,
        });
        
        // Render the kustomization template
        let kustomization = self.template_manager.render_with_serializable(
            "kustomization.yaml.tera",
            &kustomization_ctx
        )?;
        
        // Write the kustomization file
        let kustomization_file = output_dir.join("kustomization.yaml");
        std::fs::write(&kustomization_file, kustomization)
            .context("Failed to write kustomization file")?;
        
        debug!(
            workflow = %workflow.name,
            path = %kustomization_file.display(),
            "Successfully generated kustomization file"
        );
        
        Ok(kustomization_file)
    }
    
    /// Generate kustomization file for a subworkflow
    fn generate_subworkflow_kustomization(&mut self, subworkflow: &Subworkflow, output_dir: &Path) -> Result<PathBuf> {
        debug!("Generating kustomization for subworkflow: {}", subworkflow.name);
        
        // Get a list of all YAML files in the subworkflow directory
        let resource_files = std::fs::read_dir(output_dir)?
            .filter_map(|entry| {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().map_or(false, |ext| ext == "yaml") {
                        return Some(path.file_name()?.to_string_lossy().to_string());
                    }
                }
                None
            })
            .collect::<Vec<_>>();
        
        // Create context for the template
        let kustomization_ctx = serde_json::json!({
            "workflow_name": subworkflow.name,
            "namespace": self.context.namespace,
            "resource_files": resource_files,
            "is_subworkflow": true,
        });
        
        // Render the kustomization template
        let kustomization = self.template_manager.render_with_serializable(
            "kustomization.yaml.tera",
            &kustomization_ctx
        )?;
        
        // Write the kustomization file
        let kustomization_file = output_dir.join("kustomization.yaml");
        std::fs::write(&kustomization_file, kustomization)
            .context("Failed to write subworkflow kustomization file")?;
        
        debug!(
            subworkflow = %subworkflow.name,
            path = %kustomization_file.display(),
            "Successfully generated subworkflow kustomization file"
        );
        
        Ok(kustomization_file)
    }
    
    /// Generate the root kustomization file for the entire program
    fn generate_kustomization(&mut self, _program: &Program) -> Result<PathBuf> {
        info!("Generating root kustomization file");
        
        // Get a list of all workflow and subworkflow directories
        let resources = std::fs::read_dir(&self.output_dir)?
            .filter_map(|entry| {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        return Some(path.file_name()?.to_string_lossy().to_string());
                    }
                }
                None
            })
            .collect::<Vec<_>>();
        
        // Add NATS resources
        let mut all_resources = vec!["nats".to_string()];
        all_resources.extend(resources);
        
        // Create context for the template
        let kustomization_ctx = serde_json::json!({
            "namespace": self.context.namespace,
            "project_name": self.context.project_name,
            "resources": all_resources,
        });
        
        // Render the root kustomization template
        let kustomization = self.template_manager.render_with_serializable(
            "root/kustomization.yaml.tera",
            &kustomization_ctx
        )?;
        
        // Write the root kustomization file
        let kustomization_file = self.output_dir.join("kustomization.yaml");
        std::fs::write(&kustomization_file, kustomization)
            .context("Failed to write root kustomization file")?;
        
        info!(
            path = %kustomization_file.display(),
            "Successfully generated root kustomization file"
        );
        
        Ok(kustomization_file)
    }
}

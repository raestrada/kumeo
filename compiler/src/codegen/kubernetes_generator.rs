use std::collections::HashMap;
use std::path::{Path, PathBuf};
use serde::Serialize;
use anyhow::Context;
use crate::ast::{Program, Workflow, Subworkflow, Agent, AgentType, Value, Argument};
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
#[derive(Debug, Clone, Serialize)]
struct WorkflowContext {
    name: String,
    namespace: String,
    agents: Vec<AgentContext>,
}

/// Context for subworkflow manifest generation
#[derive(Debug, Clone, Serialize)]
struct SubworkflowContext {
    name: String,
    namespace: String,
    agents: Vec<AgentContext>,
}

/// Context for agent manifest generation
#[derive(Debug, Clone, Serialize)]
struct AgentContext {
    name: String,
    namespace: String,
    image: String,
    input_topic: Option<String>,
    output_topic: Option<String>,
    config: Vec<crate::ast::Argument>,
    resources: ResourceRequirements,
}

/// Resource requirements for Kubernetes containers
#[derive(Debug, Clone, Serialize)]
struct ResourceRequirements {
    requests: ResourceQuantity,
    limits: ResourceQuantity,
}

/// Resource quantity for Kubernetes
#[derive(Debug, Clone, Serialize)]
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
        info!("Generating Kubernetes manifests");
        
        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&self.output_dir).map_err(TemplateError::Io)?;
        
        // Generate namespace
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
        
        // Generate root kustomization file
        let kustomization_file = self.generate_kustomization(program)?;
        
        info!(
            path = %self.output_dir.display(),
            "Successfully generated Kubernetes manifests"
        );
        
        Ok(kustomization_file)
    }
    
    /// Generate a Kubernetes namespace for the program
    fn generate_namespace(&mut self, _program: &Program) -> Result<PathBuf> {
        debug!("Generating namespace for program");
        
        // Create a directory for the namespace
        let ns_dir = self.output_dir.join("namespace");
        std::fs::create_dir_all(&ns_dir).map_err(TemplateError::Io)?;
        
        // Create context for the template
        let namespace_ctx = serde_json::json!({
            "name": &self.context.namespace,
            "labels": {
                "app.kubernetes.io/name": "kumeo",
                "app.kubernetes.io/instance": &self.context.namespace,
            }
        });
        
        // Render the namespace template
        let namespace = self.template_manager.render_with_serializable(
            "namespace.yaml.tera",
            &namespace_ctx
        )?;
        
        // Write the namespace manifest to a file
        let namespace_file = ns_dir.join("namespace.yaml");
        std::fs::write(&namespace_file, namespace)
            .map_err(TemplateError::Io)?;
        
        debug!(
            namespace = %self.context.namespace,
            path = %namespace_file.display(),
            "Successfully generated namespace manifest"
        );
        
        // Generate kustomization file
        let kustomization_ctx = serde_json::json!({
            "namespace": &self.context.namespace,
            "resources": ["namespace.yaml"],
        });
        
        let kustomization = self.template_manager.render_with_serializable(
            "kustomization.yaml.tera",
            &kustomization_ctx
        )?;
        
        let kustomization_file = ns_dir.join("kustomization.yaml");
        std::fs::write(&kustomization_file, kustomization)
            .map_err(TemplateError::Io)?;
        
        Ok(ns_dir)
    }
    
    /// Generate NATS infrastructure manifests
    fn generate_nats_infrastructure(&mut self) -> Result<PathBuf> {
        debug!("Generating NATS infrastructure manifests");
        
        // Create a directory for NATS infrastructure
        let nats_dir = self.output_dir.join("nats");
        std::fs::create_dir_all(&nats_dir).map_err(TemplateError::Io)?;
        
        // Create context for the template
        let nats_ctx = serde_json::json!({
            "namespace": &self.context.namespace,
            "nats_image": "nats:2.9.0",
            "nats_cluster_name": "kumeo-nats",
            "nats_cluster_size": 3,
            "nats_http_port": 8222,
            "nats_cluster_port": 6222,
            "nats_leaf_port": 7422,
            "nats_gateway_port": 7522,
        });
        
        // Render the NATS statefulset template
        let statefulset = self.template_manager.render_with_serializable(
            "nats/statefulset.yaml.tera",
            &nats_ctx
        )?;
        
        // Write the statefulset manifest to a file
        let statefulset_file = nats_dir.join("statefulset.yaml");
        std::fs::write(&statefulset_file, statefulset)
            .map_err(TemplateError::Io)?;
        
        // Render the NATS service template
        let service = self.template_manager.render_with_serializable(
            "nats/service.yaml.tera",
            &nats_ctx
        )?;
        
        // Write the service manifest to a file
        let service_file = nats_dir.join("service.yaml");
        std::fs::write(&service_file, service)
            .map_err(TemplateError::Io)?;
        
        // Generate kustomization file
        let kustomization_ctx = serde_json::json!({
            "namespace": &self.context.namespace,
            "resources": ["service.yaml", "statefulset.yaml"],
        });
        
        let kustomization = self.template_manager.render_with_serializable(
            "kustomization.yaml.tera",
            &kustomization_ctx
        )?;
        
        let kustomization_file = nats_dir.join("kustomization.yaml");
        std::fs::write(&kustomization_file, kustomization)
            .map_err(TemplateError::Io)?;
        
        debug!(
            path = %nats_dir.display(),
            "Successfully generated NATS infrastructure manifests"
        );
        
        Ok(nats_dir)
    }
    
    /// Generate Kubernetes manifests for a workflow
    fn generate_workflow_manifests(&mut self, workflow: &Workflow) -> Result<PathBuf> {
        use crate::ast::Context; // Add this line to import Context
        debug!("Generating manifests for workflow: {}", workflow.name);
        
        // Create workflow directory
        let workflow_dir = self.output_dir.join("workflows").join(&workflow.name);
        std::fs::create_dir_all(&workflow_dir)?;
        
        // Create workflow context
        let workflow_ctx = WorkflowContext {
            name: workflow.name.clone(),
            namespace: self.context.namespace.clone(),
            agents: Vec::new(),
        };
        
        // Generate manifests for each agent
        for (index, agent) in workflow.agents.iter().enumerate() {
            let agent_dir = self.generate_agent_manifests(agent, workflow, &workflow_dir, index)?;
            
            // Add agent to workflow context
            let agent_ctx = AgentContext {
                name: agent.id.clone().unwrap_or_else(|| format!("agent-{}", index)),
                namespace: self.context.namespace.clone(),
                image: format!("kumeo/{}:latest", agent.agent_type.to_string().to_lowercase()),
                input_topic: workflow.source.as_ref().map(|_| "input".to_string()),
                output_topic: workflow.target.as_ref().map(|_| "output".to_string()),
                config: agent.config.clone(),
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
            
            // Clone the context to avoid borrowing issues
            let mut workflow_ctx = workflow_ctx.clone();
            workflow_ctx.agents.push(agent_ctx);
        }
        
        // Generate kustomization file
        self.generate_context_configmaps(&workflow.context, workflow, &workflow_dir)?;
        
        // Generate a kustomization file for the workflow
        self.generate_workflow_kustomization(workflow, &workflow_dir)?;
        
        Ok(workflow_dir)
    }
    
    /// Generate Kubernetes manifests for a subworkflow
    fn generate_subworkflow_manifests(&mut self, subworkflow: &Subworkflow) -> Result<PathBuf> {
        use crate::ast::Context; // Add this line to import Context
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
    fn generate_agent_manifests(
        &mut self,
        agent: &Agent,
        workflow: &Workflow,
        output_dir: &Path,
        index: usize,
    ) -> Result<PathBuf> {
        let agent_id = agent.id.as_ref().ok_or_else(|| {
            TemplateError::rendering_error("Agent is missing ID")
        })?;

        let agent_dir = output_dir.join(format!("agent-{}", agent_id));
        std::fs::create_dir_all(&agent_dir)?;

        // Create agent context
        let agent_ctx = AgentContext {
            name: agent_id.clone(),
            namespace: self.context.namespace.clone(),
            image: format!("kumeo/{}:latest", agent.agent_type.to_string().to_lowercase()),
            input_topic: workflow.source.as_ref().map(|_| "input".to_string()),
            output_topic: workflow.target.as_ref().map(|_| "output".to_string()),
            config: agent.config.clone(),
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
        output_dir: &Path,
    ) -> Result<Vec<PathBuf>> {
        let mut configmap_files = Vec::new();
        
        if let Some(ctx) = context {
            // Create a directory for context ConfigMaps
            let ctx_dir = output_dir.join("context");
            std::fs::create_dir_all(&ctx_dir).map_err(TemplateError::Io)?;
            
            // Convert Context to a serializable format
            let data = match ctx {
                crate::ast::Context::KnowledgeBase(name, params) => {
                    let mut data = HashMap::new();
                    data.insert("type".to_string(), Value::String("knowledge-base".to_string()));
                    data.insert("name".to_string(), Value::String(name.clone()));
                    if let Some(params) = params {
                        for (k, v) in params {
                            data.insert(k.clone(), v.clone());
                        }
                    }
                    data
                },
                crate::ast::Context::BayesianNetwork(name, params) => {
                    let mut data = HashMap::new();
                    data.insert("type".to_string(), Value::String("bayesian-network".to_string()));
                    data.insert("name".to_string(), Value::String(name.clone()));
                    if let Some(params) = params {
                        for (k, v) in params {
                            data.insert(k.clone(), v.clone());
                        }
                    }
                    data
                },
                crate::ast::Context::Database(conn_str, schema) => {
                    let mut data = HashMap::new();
                    data.insert("type".to_string(), Value::String("database".to_string()));
                    data.insert("connection_string".to_string(), Value::String(conn_str.clone()));
                    data.insert("schema".to_string(), Value::String(schema.clone()));
                    data
                },
                crate::ast::Context::Custom(name, args) => {
                    let mut data = HashMap::new();
                    data.insert("type".to_string(), Value::String("custom".to_string()));
                    data.insert("name".to_string(), Value::String(name.clone()));
                    data.insert("args".to_string(), Value::Array(args.clone()));
                    data
                }
            };
            
            // Create context for the template
            let configmap_ctx = serde_json::json!({
                "name": format!("{}-context", workflow.name),
                "namespace": &self.context.namespace,
                "data": data,
            });
            
            // Render the ConfigMap template
            let configmap = self.template_manager.render_with_serializable(
                "configmap.yaml.tera",
                &configmap_ctx
            )?;
            
            // Write the ConfigMap manifest to a file
            let configmap_file = ctx_dir.join("context-configmap.yaml");
            std::fs::write(&configmap_file, configmap)
                .map_err(TemplateError::Io)?;
            
            configmap_files.push(configmap_file.clone());
            
            debug!(
                workflow = %workflow.name,
                path = %configmap_file.display(),
                "Successfully generated context ConfigMap"
            );
            
            // Generate kustomization file
            let kustomization_ctx = serde_json::json!({
                "namespace": &self.context.namespace,
                "resources": ["context-configmap.yaml"],
            });
            
            let kustomization = self.template_manager.render_with_serializable(
                "kustomization.yaml.tera",
                &kustomization_ctx
            )?;
            
            let kustomization_file = ctx_dir.join("kustomization.yaml");
            std::fs::write(&kustomization_file, kustomization)
                .map_err(TemplateError::Io)?;
                
            configmap_files.push(kustomization_file);
        }
        
        Ok(configmap_files)
    }
    
    /// Generate kustomization file for a workflow
    fn generate_workflow_kustomization(
        &mut self,
        workflow: &Workflow,
        output_dir: &Path,
    ) -> Result<PathBuf> {
        debug!("Generating kustomization for workflow: {}", workflow.name);
        
        // Get all YAML files in the output directory
        let mut resource_files = Vec::new();
        
        if let Ok(entries) = std::fs::read_dir(output_dir) {
            for entry in entries.filter_map(|res: std::io::Result<std::fs::DirEntry>| res.ok()) {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Some(ext) = entry.path().extension() {
                            if ext == "yaml" || ext == "yml" {
                                if let Some(file_name) = entry.file_name().to_str() {
                                    resource_files.push(file_name.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Create context for the template
        let kustomization_ctx = serde_json::json!({
            "namespace": &self.context.namespace,
            "resources": resource_files,
        });
        
        // Render the kustomization template
        let kustomization = self.template_manager.render_with_serializable(
            "kustomization.yaml.tera",
            &kustomization_ctx
        )?;
        
        // Write the kustomization file
        let kustomization_file = output_dir.join("kustomization.yaml");
        std::fs::write(&kustomization_file, kustomization)
            .map_err(TemplateError::Io)?;
        
        debug!(
            workflow = %workflow.name,
            path = %kustomization_file.display(),
            "Successfully generated workflow kustomization"
        );
        
        Ok(kustomization_file)
    }
    
    /// Generate kustomization file for a subworkflow
    fn generate_subworkflow_kustomization(
        &mut self,
        subworkflow: &Subworkflow,
        output_dir: &Path,
    ) -> Result<PathBuf> {
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
    
    /// Generate a ConfigMap for agent configuration
    fn generate_agent_configmap(
        &mut self,
        agent: &Agent,
        workflow: &Workflow,
        output_dir: &Path,
    ) -> Result<PathBuf> {
        let agent_id = agent.id.as_deref().unwrap_or_else(|| {
            warn!("Agent is missing ID, using default");
            "default-agent"
        });
        
        // Create a ConfigMap name based on the workflow and agent
        let configmap_name = format!("{}-{}-config", workflow.name, agent_id);
        
        // Convert agent config to a serializable format
        let mut config_data = HashMap::new();
        
        // Add agent type
        config_data.insert("type".to_string(), Value::String(agent.agent_type.to_string()));
        
        // Add agent configuration
        let mut config_map = HashMap::new();
        for arg in &agent.config {
            match arg {
                Argument::Positional(value) => {
                    config_map.insert("_".to_string(), value.clone());
                }
                Argument::Named(name, value) => {
                    config_map.insert(name.clone(), value.clone());
                }
            }
        }
        
        // Create context for the template
        let configmap_ctx = serde_json::json!({
            "name": configmap_name,
            "namespace": &self.context.namespace,
            "data": config_map,
        });
        
        // Render the ConfigMap template
        let configmap = self.template_manager.render_with_serializable(
            "configmap.yaml.tera",
            &configmap_ctx
        )?;
        
        // Write the ConfigMap manifest to a file
        let configmap_file = output_dir.join("configmap.yaml");
        std::fs::write(&configmap_file, configmap)
            .map_err(TemplateError::Io)?;
        
        debug!(
            agent = %agent_id,
            workflow = %workflow.name,
            path = %configmap_file.display(),
            "Successfully generated agent ConfigMap"
        );
        
        Ok(configmap_file)
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

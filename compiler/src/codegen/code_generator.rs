use std::path::{Path, PathBuf};
use crate::ast::{Agent, AgentType, Program, Workflow, Subworkflow};
use crate::codegen::template_manager::Result;
use crate::codegen::rust_generator::RustGenerator;
use crate::codegen::python_generator::PythonGenerator;
use crate::codegen::kubernetes_generator::KubernetesGenerator;

/// Main code generator that orchestrates generation of code in multiple languages
pub struct CodeGenerator {
    output_dir: PathBuf,
    rust_generator: RustGenerator,
    python_generator: PythonGenerator,
    kubernetes_generator: KubernetesGenerator,
}

impl CodeGenerator {
    /// Create a new code generator with the given template root and output directory
    pub fn new<P: AsRef<Path>>(template_root: P, output_dir: P) -> Result<Self> {
        let template_root = template_root.as_ref().to_path_buf();
        let output_dir = output_dir.as_ref().to_path_buf();
        
        // Create output directory if it doesn't exist
        if !output_dir.exists() {
            std::fs::create_dir_all(&output_dir)?;
        }
        
        Ok(Self {
            output_dir: output_dir.clone(),
            rust_generator: RustGenerator::new(&template_root, &output_dir)?,
            python_generator: PythonGenerator::new(&template_root, &output_dir)?,
            kubernetes_generator: KubernetesGenerator::new(&template_root, &output_dir)?,
        })
    }
    
    /// Generate code for a Kumeo program
    pub fn generate(&mut self, program: &Program) -> Result<PathBuf> {
        println!("Generating code for Kumeo program...");
        
        // Create project structure
        let project_dir = self.create_project_structure()?;
        
        // Generate code for each workflow
        for workflow in &program.workflows {
            self.generate_workflow(workflow)?;
        }
        
        // Generate code for each subworkflow
        for subworkflow in &program.subworkflows {
            self.generate_subworkflow(subworkflow)?;
        }
        
        // Generate Kubernetes manifests
        self.kubernetes_generator.generate_kubernetes_manifests(program)?;
        
        Ok(project_dir)
    }
    
    /// Create the basic project structure
    fn create_project_structure(&self) -> Result<PathBuf> {
        // Create language-specific directories
        let rust_dir = self.output_dir.join("rust");
        let python_dir = self.output_dir.join("python");
        let k8s_dir = self.output_dir.join("kubernetes");
        
        std::fs::create_dir_all(&rust_dir.join("src/agents"))?;
        std::fs::create_dir_all(&rust_dir.join("src/infrastructure"))?;
        std::fs::create_dir_all(&python_dir.join("src/agents"))?;
        std::fs::create_dir_all(&python_dir.join("src/infrastructure"))?;
        std::fs::create_dir_all(&k8s_dir)?;
        
        Ok(self.output_dir.clone())
    }
    
    /// Generate code for a workflow
    fn generate_workflow(&mut self, workflow: &Workflow) -> Result<()> {
        println!("Generating code for workflow: {}", workflow.name);
        
        // Generate code for each agent in the workflow
        for agent in &workflow.agents {
            self.generate_agent(agent, workflow)?;
        }
        
        // Generate code for preprocessors if present
        if let Some(preprocessors) = &workflow.preprocessors {
            for agent in preprocessors {
                self.generate_agent(agent, workflow)?;
            }
        }
        
        Ok(())
    }
    
    /// Generate code for a subworkflow
    fn generate_subworkflow(&mut self, subworkflow: &Subworkflow) -> Result<()> {
        println!("Generating code for subworkflow: {}", subworkflow.name);
        
        // Generate code for each agent in the subworkflow
        for agent in &subworkflow.agents {
            // Create a temporary workflow-like structure for the subworkflow
            // This is needed because our agent generators expect a workflow context
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
            
            self.generate_agent(agent, &temp_workflow)?;
        }
        
        Ok(())
    }
    
    /// Generate code for an agent, selecting the appropriate language based on agent type
    fn generate_agent(&mut self, agent: &Agent, workflow: &Workflow) -> Result<()> {
        println!("Generating code for agent: {:?}", agent.id);
        
        match agent.agent_type {
            // Rust is used for these agent types
            AgentType::LLM => {
                self.rust_generator.generate_llm_agent(agent, workflow)?;
            },
            AgentType::Router => {
                self.rust_generator.generate_router_agent(agent, workflow)?;
            },
            AgentType::Aggregator => {
                self.rust_generator.generate_aggregator_agent(agent, workflow)?;
            },
            AgentType::RuleEngine => {
                self.rust_generator.generate_rule_engine_agent(agent, workflow)?;
            },
            AgentType::DataNormalizer => {
                self.rust_generator.generate_data_normalizer_agent(agent, workflow)?;
            },
            AgentType::MissingValueHandler => {
                self.rust_generator.generate_missing_value_handler_agent(agent, workflow)?;
            },
            
            // Python is used for these agent types
            AgentType::MLModel => {
                self.python_generator.generate_ml_model_agent(agent, workflow)?;
            },
            AgentType::BayesianNetwork => {
                self.python_generator.generate_bayesian_network_agent(agent, workflow)?;
            },
            AgentType::DecisionMatrix => {
                self.python_generator.generate_decision_matrix_agent(agent, workflow)?;
            },
            
            // Other agent types
            AgentType::HumanInLoop => {
                // Human-in-loop requires both a backend service and a frontend component
                self.rust_generator.generate_human_in_loop_backend(agent, workflow)?;
                // We would also generate frontend components here
            },
            AgentType::Custom(ref name) => {
                // For custom agents, we would need to determine the appropriate language
                // based on additional configuration or a default
                println!("Custom agent type: {}", name);
                // For now, default to Rust
                self.rust_generator.generate_custom_agent(agent, workflow)?;
            }
        }
        
        Ok(())
    }
}

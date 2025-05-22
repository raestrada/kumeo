use std::fs;
use std::path::PathBuf;
use kumeo_compiler::ast::{Program, Workflow, Subworkflow, Agent, AgentType, Value, Argument, Context};
use kumeo_compiler::codegen::kubernetes_generator::KubernetesGenerator;

#[test]
fn test_kubernetes_generator_initialization() {
    // Create a temporary directory for templates and output
    let temp_dir = tempfile::tempdir().unwrap();
    let template_root = temp_dir.path().join("templates");
    let output_dir = temp_dir.path().join("output");
    
    // Create template directories
    fs::create_dir_all(&template_root.join("kubernetes")).unwrap();
    
    // Create sample templates
    let namespace_template = "apiVersion: v1\nkind: Namespace\nmetadata:\n  name: {{namespace}}";
    let deployment_template = "apiVersion: apps/v1\nkind: Deployment\nmetadata:\n  name: {{agent_id}}-{{workflow_name}}\nspec:\n  selector:\n    matchLabels:\n      app: {{agent_id}}";
    let service_template = "apiVersion: v1\nkind: Service\nmetadata:\n  name: {{agent_id}}-{{workflow_name}}\nspec:\n  selector:\n    app: {{agent_id}}";
    let configmap_template = "apiVersion: v1\nkind: ConfigMap\nmetadata:\n  name: {{agent_id}}-config\ndata:\n  agent-config.json: |\n    {{config_json}}";
    let kustomization_template = "apiVersion: kustomize.config.k8s.io/v1beta1\nkind: Kustomization\nresources:\n  - {{resource_files}}";
    let root_kustomization_template = "apiVersion: kustomize.config.k8s.io/v1beta1\nkind: Kustomization\nresources:\n  - {{resource_files}}";
    let nats_template = "apiVersion: v1\nkind: Service\nmetadata:\n  name: nats\nspec:\n  selector:\n    app: nats";
    let context_configmap_template = "apiVersion: v1\nkind: ConfigMap\nmetadata:\n  name: {{workflow_name}}-context\ndata:\n  context.json: |\n    {{context_json}}";
    
    // Write templates to files
    fs::write(template_root.join("kubernetes/namespace.yaml.tmpl"), namespace_template).unwrap();
    fs::write(template_root.join("kubernetes/deployment.yaml.tmpl"), deployment_template).unwrap();
    fs::write(template_root.join("kubernetes/service.yaml.tmpl"), service_template).unwrap();
    fs::write(template_root.join("kubernetes/configmap.yaml.tmpl"), configmap_template).unwrap();
    fs::write(template_root.join("kubernetes/kustomization.yaml.tmpl"), kustomization_template).unwrap();
    fs::write(template_root.join("kubernetes/root-kustomization.yaml.tmpl"), root_kustomization_template).unwrap();
    fs::write(template_root.join("kubernetes/nats.yaml.tmpl"), nats_template).unwrap();
    fs::write(template_root.join("kubernetes/context-configmap.yaml.tmpl"), context_configmap_template).unwrap();
    
    // Initialize the Kubernetes generator
    let kubernetes_generator = KubernetesGenerator::new(&template_root, &output_dir);
    assert!(kubernetes_generator.is_ok(), "Failed to initialize Kubernetes generator");
}

#[test]
fn test_kubernetes_manifest_generation() {
    // Create a temporary directory for templates and output
    let temp_dir = tempfile::tempdir().unwrap();
    let template_root = temp_dir.path().join("templates");
    let output_dir = temp_dir.path().join("output");
    
    // Create template directories
    fs::create_dir_all(&template_root.join("kubernetes")).unwrap();
    
    // Create sample templates
    let namespace_template = "apiVersion: v1\nkind: Namespace\nmetadata:\n  name: {{namespace}}";
    let deployment_template = "apiVersion: apps/v1\nkind: Deployment\nmetadata:\n  name: {{agent_id}}-{{workflow_name}}\nspec:\n  selector:\n    matchLabels:\n      app: {{agent_id}}";
    let service_template = "apiVersion: v1\nkind: Service\nmetadata:\n  name: {{agent_id}}-{{workflow_name}}\nspec:\n  selector:\n    app: {{agent_id}}";
    let configmap_template = "apiVersion: v1\nkind: ConfigMap\nmetadata:\n  name: {{agent_id}}-config\ndata:\n  agent-config.json: |\n    {{config_json}}";
    let kustomization_template = "apiVersion: kustomize.config.k8s.io/v1beta1\nkind: Kustomization\nresources:\n  - {{resource_files}}";
    let root_kustomization_template = "apiVersion: kustomize.config.k8s.io/v1beta1\nkind: Kustomization\nresources:\n  - {{resource_files}}";
    let nats_template = "apiVersion: v1\nkind: Service\nmetadata:\n  name: nats\nspec:\n  selector:\n    app: nats";
    let context_configmap_template = "apiVersion: v1\nkind: ConfigMap\nmetadata:\n  name: {{workflow_name}}-context\ndata:\n  context.json: |\n    {{context_json}}";
    
    // Write templates to files
    fs::write(template_root.join("kubernetes/namespace.yaml.tmpl"), namespace_template).unwrap();
    fs::write(template_root.join("kubernetes/deployment.yaml.tmpl"), deployment_template).unwrap();
    fs::write(template_root.join("kubernetes/service.yaml.tmpl"), service_template).unwrap();
    fs::write(template_root.join("kubernetes/configmap.yaml.tmpl"), configmap_template).unwrap();
    fs::write(template_root.join("kubernetes/kustomization.yaml.tmpl"), kustomization_template).unwrap();
    fs::write(template_root.join("kubernetes/root-kustomization.yaml.tmpl"), root_kustomization_template).unwrap();
    fs::write(template_root.join("kubernetes/nats.yaml.tmpl"), nats_template).unwrap();
    fs::write(template_root.join("kubernetes/context-configmap.yaml.tmpl"), context_configmap_template).unwrap();
    
    // Initialize the Kubernetes generator
    let mut kubernetes_generator = KubernetesGenerator::new(&template_root, &output_dir).unwrap();
    
    // Create a test program with workflow and agents
    let program = create_test_program();
    
    // Generate Kubernetes manifests
    let result = kubernetes_generator.generate_kubernetes_manifests(&program);
    assert!(result.is_ok(), "Failed to generate Kubernetes manifests");
    
    // Verify that the output directory exists and contains the expected files
    let k8s_dir = result.unwrap();
    assert!(k8s_dir.exists(), "Kubernetes output directory was not created");
    
    // Check if namespace.yaml was created
    let namespace_file = k8s_dir.join("namespace.yaml");
    assert!(namespace_file.exists(), "Namespace manifest was not created");
    
    // Check if nats.yaml was created
    let nats_file = k8s_dir.join("nats.yaml");
    assert!(nats_file.exists(), "NATS manifest was not created");
    
    // Check if kustomization.yaml was created
    let kustomization_file = k8s_dir.join("kustomization.yaml");
    assert!(kustomization_file.exists(), "Root kustomization manifest was not created");
    
    // Check if workflow directory was created
    let workflow_dir = k8s_dir.join("TestWorkflow");
    assert!(workflow_dir.exists(), "Workflow directory was not created");
    
    // Check if agent manifests were created for workflow
    let llm_deployment_file = workflow_dir.join("testllm-deployment.yaml");
    assert!(llm_deployment_file.exists(), "LLM agent deployment manifest was not created");
    
    let llm_service_file = workflow_dir.join("testllm-service.yaml");
    assert!(llm_service_file.exists(), "LLM agent service manifest was not created");
    
    let llm_configmap_file = workflow_dir.join("testllm-configmap.yaml");
    assert!(llm_configmap_file.exists(), "LLM agent configmap manifest was not created");
    
    // Check if subworkflow directory was created
    let subworkflow_dir = k8s_dir.join("TestSubworkflow");
    assert!(subworkflow_dir.exists(), "Subworkflow directory was not created");
    
    // Check if agent manifests were created for subworkflow
    let ml_deployment_file = subworkflow_dir.join("testmlmodel-deployment.yaml");
    assert!(ml_deployment_file.exists(), "ML Model agent deployment manifest was not created");
    
    let ml_service_file = subworkflow_dir.join("testmlmodel-service.yaml");
    assert!(ml_service_file.exists(), "ML Model agent service manifest was not created");
    
    let ml_configmap_file = subworkflow_dir.join("testmlmodel-configmap.yaml");
    assert!(ml_configmap_file.exists(), "ML Model agent configmap manifest was not created");
}

#[test]
fn test_kubernetes_manifest_content() {
    // Create a temporary directory for templates and output
    let temp_dir = tempfile::tempdir().unwrap();
    let template_root = temp_dir.path().join("templates");
    let output_dir = temp_dir.path().join("output");
    
    // Create template directories
    fs::create_dir_all(&template_root.join("kubernetes")).unwrap();
    
    // Create sample templates with more detailed content to test parameter substitution
    let deployment_template = r#"apiVersion: apps/v1
kind: Deployment
metadata:
  name: {{agent_id}}-{{workflow_name}}
  labels:
    app: {{agent_id}}
    workflow: {{workflow_name}}
    kumeo.io/agent-type: {{agent_type}}
spec:
  replicas: 1
  selector:
    matchLabels:
      app: {{agent_id}}
      workflow: {{workflow_name}}
  template:
    metadata:
      labels:
        app: {{agent_id}}
        workflow: {{workflow_name}}
    spec:
      containers:
      - name: {{agent_id}}
        image: kumeo/{{language}}-agent:latest
        env:
        - name: AGENT_ID
          value: "{{agent_id}}"
        - name: WORKFLOW_NAME
          value: "{{workflow_name}}"
"#;
    
    // Create other required templates
    let namespace_template = "apiVersion: v1\nkind: Namespace\nmetadata:\n  name: {{namespace}}";
    let service_template = "apiVersion: v1\nkind: Service\nmetadata:\n  name: {{agent_id}}-{{workflow_name}}\nspec:\n  selector:\n    app: {{agent_id}}";
    let configmap_template = "apiVersion: v1\nkind: ConfigMap\nmetadata:\n  name: {{agent_id}}-config\ndata:\n  agent-config.json: |\n    {{config_json}}";
    let kustomization_template = "apiVersion: kustomize.config.k8s.io/v1beta1\nkind: Kustomization\nresources:\n  - {{resource_files}}";
    let root_kustomization_template = "apiVersion: kustomize.config.k8s.io/v1beta1\nkind: Kustomization\nresources:\n  - {{resource_files}}";
    let nats_template = "apiVersion: v1\nkind: Service\nmetadata:\n  name: nats\nspec:\n  selector:\n    app: nats";
    let context_configmap_template = "apiVersion: v1\nkind: ConfigMap\nmetadata:\n  name: {{workflow_name}}-context\ndata:\n  context.json: |\n    {{context_json}}";
    
    // Write templates to files
    fs::write(template_root.join("kubernetes/namespace.yaml.tmpl"), namespace_template).unwrap();
    fs::write(template_root.join("kubernetes/deployment.yaml.tmpl"), deployment_template).unwrap();
    fs::write(template_root.join("kubernetes/service.yaml.tmpl"), service_template).unwrap();
    fs::write(template_root.join("kubernetes/configmap.yaml.tmpl"), configmap_template).unwrap();
    fs::write(template_root.join("kubernetes/kustomization.yaml.tmpl"), kustomization_template).unwrap();
    fs::write(template_root.join("kubernetes/root-kustomization.yaml.tmpl"), root_kustomization_template).unwrap();
    fs::write(template_root.join("kubernetes/nats.yaml.tmpl"), nats_template).unwrap();
    fs::write(template_root.join("kubernetes/context-configmap.yaml.tmpl"), context_configmap_template).unwrap();
    
    // Initialize the Kubernetes generator
    let mut kubernetes_generator = KubernetesGenerator::new(&template_root, &output_dir).unwrap();
    
    // Create a test program with workflow and agents
    let program = create_test_program();
    
    // Generate Kubernetes manifests
    let k8s_dir = kubernetes_generator.generate_kubernetes_manifests(&program).unwrap();
    
    // Check content of deployment file for LLM agent
    let workflow_dir = k8s_dir.join("TestWorkflow");
    let llm_deployment_file = workflow_dir.join("testllm-deployment.yaml");
    let deployment_content = fs::read_to_string(llm_deployment_file).unwrap();
    
    // Print the deployment content for debugging
    println!("Deployment content:\n{}", deployment_content);
    
    // Verify the deployment content contains the expected substituted values
    assert!(deployment_content.contains("name: testllm") || deployment_content.contains("name: TestLLM"), "Agent name not found in deployment");
    assert!(deployment_content.contains("app: TestLLM"), "Agent label not found in deployment");
    assert!(deployment_content.contains("workflow: TestWorkflow"), "Workflow label not found in deployment");
    assert!(deployment_content.contains("value: \"TestLLM\""), "Agent ID env var not found in deployment");
    assert!(deployment_content.contains("value: \"TestWorkflow\""), "Workflow name env var not found in deployment");
    
    // For rust agent, check if the language is set correctly
    assert!(deployment_content.contains("image: kumeo/rust-agent:latest"), "Rust image not found in deployment");
    
    // Check that the name field in the container is set correctly
    assert!(deployment_content.contains("name: TestLLM"), "Container name not set correctly");
    
    // Check content of deployment file for ML Model agent in subworkflow
    let subworkflow_dir = k8s_dir.join("TestSubworkflow");
    let ml_deployment_file = subworkflow_dir.join("testmlmodel-deployment.yaml");
    let ml_deployment_content = fs::read_to_string(ml_deployment_file).unwrap();
    
    // Print the ML deployment content for debugging
    println!("ML Deployment content:\n{}", ml_deployment_content);
    
    // Debugging output for troubleshooting
    if !ml_deployment_content.contains("TestMLModel") {
        println!("ERROR: TestMLModel not found in deployment content");
    }
    
    // Verify the deployment content contains the expected substituted values
    assert!(ml_deployment_content.contains("name: testmlmodel") || ml_deployment_content.contains("name: TestMLModel"), "Agent name not found in ML deployment");
    assert!(ml_deployment_content.contains("app: TestMLModel"), "Agent label not found in ML deployment");
    assert!(ml_deployment_content.contains("workflow: TestSubworkflow"), "Subworkflow label not found in ML deployment");
    
    // For python agent, check if the language is set correctly
    assert!(ml_deployment_content.contains("image: kumeo/python-agent:latest"), "Python image not found in deployment");
}

// Helper function to create a test program for Kubernetes manifest generation
fn create_test_program() -> Program {
    // Create LLM agent for workflow
    let llm_agent = Agent {
        id: Some("TestLLM".to_string()),
        agent_type: AgentType::LLM,
        config: vec![
            Argument::Named("engine".to_string(), Value::String("gpt-4".to_string())),
            Argument::Named("prompt".to_string(), Value::String("Hello, world!".to_string())),
        ],
    };
    
    // Create ML Model agent for subworkflow
    let ml_agent = Agent {
        id: Some("TestMLModel".to_string()),
        agent_type: AgentType::MLModel,
        config: vec![
            Argument::Named("model_path".to_string(), Value::String("/path/to/model.pkl".to_string())),
        ],
    };
    
    // Create workflow
    let workflow = Workflow {
        name: "TestWorkflow".to_string(),
        source: None,
        target: None,
        context: None,
        preprocessors: None,
        agents: vec![llm_agent],
        monitor: None,
        deployment: None,
    };
    
    // Create subworkflow
    let subworkflow = Subworkflow {
        name: "TestSubworkflow".to_string(),
        input: None,
        output: None,
        context: None,
        agents: vec![ml_agent],
    };
    
    // Create program
    Program {
        workflows: vec![workflow],
        subworkflows: vec![subworkflow],
        integrations: vec![],
    }
}

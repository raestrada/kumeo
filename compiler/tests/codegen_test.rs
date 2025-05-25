//! Integration tests for the code generation system

use kumeo_compiler::ast::Program;
use std::fs;
use std::path::Path;

mod test_utils;
use test_utils::{create_test_program, create_test_generator};

#[test]
fn test_generate_agent_code() {
    // Create test data
    let program = create_test_program();
    let (mut generator, test_env) = create_test_generator();
    
    // Generate the code
    generator.generate(&program).expect("Code generation failed");
    
    // Verify the output files were created
    let rust_agent_file = test_env.output_dir.join("src/agents/test_agent.rs");
    assert!(rust_agent_file.exists(), "Rust agent file was not created");
    
    // Verify the agent code contains the expected content
    if let Ok(agent_code) = std::fs::read_to_string(&rust_agent_file) {
        // Verify the agent struct was generated
        assert!(
            agent_code.contains("struct TestAgent") || agent_code.contains("struct test_agent"),
            "Agent struct not found in generated code"
        );
    } else {
        panic!("Failed to read agent code from {:?}", rust_agent_file);
    }
}

#[test]
fn test_generate_kubernetes_manifests() {
    // Create test data
    let program = create_test_program();
    let (mut generator, test_env) = create_test_generator();
    
    // Generate the code
    generator.generate(&program).expect("Code generation failed");
    
    // Verify the Kubernetes manifests were created
    let k8s_dir = test_env.output_dir.join("kubernetes");
    assert!(k8s_dir.exists(), "Kubernetes directory was not created");
    
    // Check for required Kubernetes resources
    let deployment_file = k8s_dir.join("deployment.yaml");
    assert!(deployment_file.exists(), "Deployment file was not created");
    
    // Verify the deployment contains the expected agent configuration
    if let Ok(deployment) = std::fs::read_to_string(&deployment_file) {
        // Check for either test_agent or test-agent (k8s naming conventions)
        assert!(
            deployment.contains("test_agent") || deployment.contains("test-agent"),
            "Deployment does not contain the agent name"
        );
    } else {
        panic!("Failed to read deployment file from {:?}", deployment_file);
    }
}

#[test]
fn test_complete_workflow() {
    // Skip this test in CI environments since it requires Docker and Kubernetes
    if std::env::var("CI").is_ok() {
        return;
    }
    
    // Create test data
    let program = create_test_program();
    let (mut generator, test_env) = create_test_generator();
    
    // Generate the code
    generator.generate(&program).expect("Code generation failed");
    
    // Verify the output files were created
    let expected_files = [
        "src/agents/test_agent.rs",
        "Cargo.toml",
        "kubernetes/deployment.yaml",
        "kubernetes/service.yaml",
        "kubernetes/configmap.yaml"
    ];
    
    for file in &expected_files {
        let path = test_env.output_dir.join(file);
        assert!(path.exists(), "Expected file was not created: {}", file);
    }
}

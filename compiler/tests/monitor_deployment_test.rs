use kumeo_compiler::ast::Value;
use kumeo_compiler::parse;
use std::collections::HashMap;

#[test]
fn test_monitor_deployment_parsing() {
    // Ejemplo con configuraciones de Monitor y Deployment
    let monitor_deployment_example = r#"
workflow ProductionWorkflow {
    source: NATS("production-input")
    target: NATS("production-output")
    
    agents: [
        LLM(
            id: "production_llm",
            engine: "ollama/llama3",
            prompt: "Process the following production data: {{data}}"
        )
    ]
    
    monitor: {
        log_level: "info",
        metrics_enabled: true,
        alert_on_error: true,
        alert_threshold: 3,
        alert_channels: {
            slack: "alerts-channel",
            email: "alerts@example.com"
        }
    }
    
    deployment: {
        replicas: 2,
        memory: "2Gi",
        cpu: "500m",
        scaling: {
            min_replicas: 1,
            max_replicas: 5,
            target_cpu_utilization: 80
        },
        environment: "production",
        namespace: "kumeo-prod"
    }
}

workflow DevelopmentWorkflow {
    source: NATS("dev-input")
    target: NATS("dev-output")
    
    agents: [
        MLModel(
            id: "dev_model",
            model_path: "models/dev/classifier",
            batch_size: 1
        )
    ]
    
    monitor: {
        log_level: "debug",
        metrics_enabled: true,
        trace_enabled: true
    }
    
    deployment: {
        replicas: 1,
        memory: "1Gi",
        cpu: "200m",
        namespace: "kumeo-dev"
    }
}
    "#;
    
    // Parsear el ejemplo
    let program = parse(monitor_deployment_example).expect("Failed to parse Kumeo workflows with monitor and deployment");
    
    // Verificar la cantidad de workflows
    assert_eq!(program.workflows.len(), 2, "Should parse 2 workflows");
    
    // Verificar el primer workflow (ProductionWorkflow)
    let prod_workflow = &program.workflows[0];
    assert_eq!(prod_workflow.name, "ProductionWorkflow");
    
    // Verificar configuración de Monitor
    let prod_monitor = prod_workflow.monitor.as_ref().expect("Production workflow should have monitor config");
    assert_eq!(prod_monitor.len(), 5, "Production monitor should have 5 configuration items");
    
    // Verificar valores específicos del monitor
    match prod_monitor.get("log_level") {
        Some(Value::String(level)) => assert_eq!(level, "info"),
        _ => panic!("log_level should be a string with value 'info'")
    }
    
    match prod_monitor.get("metrics_enabled") {
        Some(Value::Boolean(enabled)) => assert!(enabled, "metrics_enabled should be true"),
        _ => panic!("metrics_enabled should be a boolean")
    }
    
    match prod_monitor.get("alert_threshold") {
        Some(Value::Number(threshold)) => assert_eq!(*threshold, 3.0),
        _ => panic!("alert_threshold should be a number with value 3")
    }
    
    // Verificar objeto anidado
    match prod_monitor.get("alert_channels") {
        Some(Value::Object(channels)) => {
            match channels.get("slack") {
                Some(Value::String(channel)) => assert_eq!(channel, "alerts-channel"),
                _ => panic!("slack channel should be a string")
            }
            match channels.get("email") {
                Some(Value::String(email)) => assert_eq!(email, "alerts@example.com"),
                _ => panic!("email should be a string")
            }
        },
        _ => panic!("alert_channels should be an object")
    }
    
    // Verificar configuración de Deployment
    let prod_deployment = prod_workflow.deployment.as_ref().expect("Production workflow should have deployment config");
    assert_eq!(prod_deployment.len(), 6, "Production deployment should have 6 configuration items");
    
    // Verificar valores específicos del deployment
    match prod_deployment.get("replicas") {
        Some(Value::Number(replicas)) => assert_eq!(*replicas, 2.0),
        _ => panic!("replicas should be a number with value 2")
    }
    
    match prod_deployment.get("memory") {
        Some(Value::String(memory)) => assert_eq!(memory, "2Gi"),
        _ => panic!("memory should be a string with value '2Gi'")
    }
    
    // Verificar objeto anidado en deployment
    match prod_deployment.get("scaling") {
        Some(Value::Object(scaling)) => {
            match scaling.get("min_replicas") {
                Some(Value::Number(min)) => assert_eq!(*min, 1.0),
                _ => panic!("min_replicas should be a number")
            }
            match scaling.get("max_replicas") {
                Some(Value::Number(max)) => assert_eq!(*max, 5.0),
                _ => panic!("max_replicas should be a number")
            }
        },
        _ => panic!("scaling should be an object")
    }
    
    // Verificar el segundo workflow (DevelopmentWorkflow)
    let dev_workflow = &program.workflows[1];
    assert_eq!(dev_workflow.name, "DevelopmentWorkflow");
    
    // Verificar configuración de Monitor
    let dev_monitor = dev_workflow.monitor.as_ref().expect("Development workflow should have monitor config");
    assert_eq!(dev_monitor.len(), 3, "Development monitor should have 3 configuration items");
    
    // Verificar configuración de Deployment
    let dev_deployment = dev_workflow.deployment.as_ref().expect("Development workflow should have deployment config");
    assert_eq!(dev_deployment.len(), 4, "Development deployment should have 4 configuration items");
    
    // Verificar valores específicos del deployment de desarrollo
    match dev_deployment.get("replicas") {
        Some(Value::Number(replicas)) => assert_eq!(*replicas, 1.0),
        _ => panic!("dev replicas should be a number with value 1")
    }
}

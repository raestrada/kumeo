use kumeo_compiler::parse;
use std::fs;
use std::path::Path;

fn main() {
    println!("Kumeo Monitor and Deployment Parser Test");
    println!("=======================================\n");
    
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
    
    // Crear un archivo de prueba
    let test_file = "monitor_deployment_test.kumeo";
    fs::write(test_file, monitor_deployment_example).expect("Failed to write test file");
    
    println!("Test Input:\n{}", monitor_deployment_example);
    
    // Parsear el ejemplo
    match parse(monitor_deployment_example) {
        Ok(program) => {
            println!("\n✅ Successfully parsed Kumeo workflows with monitor and deployment!");
            println!("Parsed AST Structure:");
            println!("Workflows: {}", program.workflows.len());
            
            for (i, workflow) in program.workflows.iter().enumerate() {
                println!("\nWorkflow #{}: {}", i+1, workflow.name);
                
                if let Some(source) = &workflow.source {
                    println!("  Source: {:?}", source);
                }
                
                if let Some(target) = &workflow.target {
                    println!("  Target: {:?}", target);
                }
                
                println!("  Agents: {}", workflow.agents.len());
                for (j, agent) in workflow.agents.iter().enumerate() {
                    println!("    Agent #{}: {:?}", j+1, agent);
                }
                
                if let Some(monitor) = &workflow.monitor {
                    println!("  Monitor Configuration:");
                    for (key, value) in monitor {
                        println!("    {}: {:?}", key, value);
                    }
                }
                
                if let Some(deployment) = &workflow.deployment {
                    println!("  Deployment Configuration:");
                    for (key, value) in deployment {
                        println!("    {}: {:?}", key, value);
                    }
                }
            }
        },
        Err(err) => {
            println!("\n❌ Failed to parse Kumeo workflows with monitor and deployment");
            println!("Error: {:?}", err);
        }
    }
    
    // Eliminar el archivo de prueba
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).expect("Failed to remove test file");
    }
}

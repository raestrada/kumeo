use kumeo_compiler::ast::{Program, Context};
use kumeo_compiler::parse;
use std::fs;
use std::path::Path;

fn main() {
    println!("Kumeo Context Parser Test");
    println!("=========================\n");
    
    // Ejemplo con diferentes tipos de contexto
    let context_example = r#"
workflow ContextTestWorkflow {
    source: NATS("input-events")
    target: NATS("output-events")
    
    context: KnowledgeBase("spanish-language")
    
    agents: [
        LLM(
            id: "text_processor",
            engine: "ollama/llama3",
            prompt: "Analyze the following text: {{data}}"
        )
    ]
}

workflow DatabaseContextWorkflow {
    source: NATS("db-events")
    target: NATS("db-results")
    
    context: Database("postgres", "postgresql://user:pass@localhost:5432/mydb")
    
    agents: [
        MLModel(
            id: "data_classifier",
            model_path: "models/classifier"
        )
    ]
}

workflow BayesNetContextWorkflow {
    source: NATS("probability-events")
    target: NATS("probability-results")
    
    context: BayesianNetwork("medical-diagnosis")
    
    agents: [
        BayesianNetwork(
            id: "diagnosis_engine",
            network_path: "models/bayesian/diagnosis"
        )
    ]
}
    "#;
    
    // Crear un archivo de prueba
    let test_file = "context_test.kumeo";
    fs::write(test_file, context_example).expect("Failed to write test file");
    
    println!("Test Input:\n{}", context_example);
    
    // Parsear el ejemplo
    match parse(context_example) {
        Ok(program) => {
            println!("\n✅ Successfully parsed Kumeo workflows with context!");
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
                
                if let Some(context) = &workflow.context {
                    println!("  Context: {:?}", context);
                }
                
                println!("  Agents: {}", workflow.agents.len());
                for (j, agent) in workflow.agents.iter().enumerate() {
                    println!("    Agent #{}: {:?}", j+1, agent);
                }
            }
        },
        Err(err) => {
            println!("\n❌ Failed to parse Kumeo workflows with context");
            println!("Error: {:?}", err);
        }
    }
    
    // Eliminar el archivo de prueba
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).expect("Failed to remove test file");
    }
}

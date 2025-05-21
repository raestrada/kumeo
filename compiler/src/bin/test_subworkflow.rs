use kumeo_compiler::parse;
use std::fs;
use std::path::Path;

fn main() {
    println!("Kumeo Subworkflow Parser Test");
    println!("=============================\n");
    
    // Ejemplo con subworkflows y sus parámetros
    let subworkflow_example = r#"
workflow MainWorkflow {
    source: NATS("main-input")
    target: NATS("main-output")
    
    agents: [
        LLM(
            id: "main_processor",
            engine: "ollama/llama3",
            prompt: "Process this data: {{data}}"
        )
    ]
}

subworkflow SentimentAnalysis {
    input: ["text", "language"]
    output: ["sentiment", "confidence"]
    
    context: KnowledgeBase("sentiment-context")
    
    agents: [
        MLModel(
            id: "sentiment_model",
            model_path: "models/sentiment",
            language: "multilingual"
        )
    ]
}

subworkflow EntityExtraction {
    input: ["document"]
    output: ["entities", "categories"]
    
    agents: [
        LLM(
            id: "entity_extractor",
            engine: "ollama/mistral",
            prompt: "Extract all named entities from the following text: {{document}}"
        ),
        MLModel(
            id: "entity_classifier",
            model_path: "models/classifier"
        )
    ]
}
    "#;
    
    // Crear un archivo de prueba
    let test_file = "subworkflow_test.kumeo";
    fs::write(test_file, subworkflow_example).expect("Failed to write test file");
    
    println!("Test Input:\n{}", subworkflow_example);
    
    // Parsear el ejemplo
    match parse(subworkflow_example) {
        Ok(program) => {
            println!("\n✅ Successfully parsed Kumeo program with subworkflows!");
            println!("Parsed AST Structure:");
            println!("Workflows: {}", program.workflows.len());
            println!("Subworkflows: {}", program.subworkflows.len());
            
            // Mostrar información de los workflows
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
            }
            
            // Mostrar información de los subworkflows
            for (i, subworkflow) in program.subworkflows.iter().enumerate() {
                println!("\nSubworkflow #{}: {}", i+1, subworkflow.name);
                
                if let Some(input) = &subworkflow.input {
                    println!("  Input: {:?}", input);
                }
                
                if let Some(output) = &subworkflow.output {
                    println!("  Output: {:?}", output);
                }
                
                if let Some(context) = &subworkflow.context {
                    println!("  Context: {:?}", context);
                }
                
                println!("  Agents: {}", subworkflow.agents.len());
                for (j, agent) in subworkflow.agents.iter().enumerate() {
                    println!("    Agent #{}: {:?}", j+1, agent);
                }
            }
        },
        Err(err) => {
            println!("\n❌ Failed to parse Kumeo program with subworkflows");
            println!("Error: {:?}", err);
        }
    }
    
    // Eliminar el archivo de prueba
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).expect("Failed to remove test file");
    }
}

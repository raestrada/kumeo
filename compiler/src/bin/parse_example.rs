use kumeo_compiler::parse;
use std::fs;
use std::path::Path;

fn main() {
    println!("Kumeo Parser Test with LALRPOP");
    println!("=============================\n");
    
    // Example Kumeo workflow
    let example = r#"
workflow SimpleWorkflow {
    source: NATS("input-events")
    target: NATS("output-events")
    
    agents: [
        LLM(
            id: "text_processor", 
            engine: "ollama/llama3",
            prompt: "Analyze the following text: {{data}}"
        )
    ]
}
    "#;
    
    // Create a test file
    let test_file = "example_workflow.kumeo";
    fs::write(test_file, example).expect("Failed to write test file");
    
    println!("Test Input:\n{}", example);
    
    // Parse the example
    match parse(example) {
        Ok(program) => {
            println!("\n✅ Successfully parsed Kumeo workflow!");
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
            }
        },
        Err(err) => {
            println!("\n❌ Failed to parse Kumeo workflow");
            println!("Error: {:?}", err);
        }
    }
    
    // Clean up test file
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).expect("Failed to remove test file");
    }
}

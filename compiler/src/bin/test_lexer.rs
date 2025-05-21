use kumeo_compiler::lexer::{Lexer, Token};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Kumeo Lexer Test");
    println!("===============\n");
    
    // Simple test input
    let test_input = r#"
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
    
    println!("Test Input:");
    println!("{}", test_input);
    println!("\nTokenizing...");
    
    // Initialize the lexer and tokenize the input
    let lexer = Lexer::new(test_input)?;
    
    // Print all tokens
    println!("\nTokens:");
    println!("{:<10} {:<20} {:<10} {:<10}", "Line", "Token", "Start", "End");
    println!("{:-<50}", "");
    
    for token in lexer.get_tokens() {
        println!("{:<10} {:<20} {:<10} {:<10}",
                 token.line,
                 format!("{:?}", token.token),
                 token.span.start,
                 token.span.end);
    }
    
    println!("\n✅ Lexer test completed successfully");
    
    Ok(())
}

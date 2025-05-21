use kumeo_compiler::parse;

#[test]
fn test_lexer_works_with_parser() {
    // Simple test to verify that the lexer works as expected with the parser
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
    
    // We just verify that the parser (which uses the lexer) can parse this input
    let result = parse(test_input);
    assert!(result.is_ok(), "The parser should be able to parse the input");
}

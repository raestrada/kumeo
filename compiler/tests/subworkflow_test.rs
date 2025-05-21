use kumeo_compiler::parse;

#[test]
fn test_subworkflow_parsing() {
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
    
    // Parsear el ejemplo
    let program = parse(subworkflow_example).expect("Failed to parse Kumeo program with subworkflows");
    
    // Verificar la cantidad de workflows y subworkflows
    assert_eq!(program.workflows.len(), 1, "Should parse 1 main workflow");
    assert_eq!(program.subworkflows.len(), 2, "Should parse 2 subworkflows");
    
    // Verificar el workflow principal
    let main_workflow = &program.workflows[0];
    assert_eq!(main_workflow.name, "MainWorkflow");
    assert_eq!(main_workflow.agents.len(), 1, "Main workflow should have 1 agent");
    
    // Verificar el primer subworkflow (SentimentAnalysis)
    let sentiment_subworkflow = &program.subworkflows[0];
    assert_eq!(sentiment_subworkflow.name, "SentimentAnalysis");
    
    // Verificar input/output del primer subworkflow
    if let Some(input) = &sentiment_subworkflow.input {
        assert_eq!(input.len(), 2, "SentimentAnalysis should have 2 input parameters");
        assert_eq!(input[0], "text");
        assert_eq!(input[1], "language");
    } else {
        panic!("SentimentAnalysis should have input parameters");
    }
    
    if let Some(output) = &sentiment_subworkflow.output {
        assert_eq!(output.len(), 2, "SentimentAnalysis should have 2 output parameters");
        assert_eq!(output[0], "sentiment");
        assert_eq!(output[1], "confidence");
    } else {
        panic!("SentimentAnalysis should have output parameters");
    }
    
    assert!(sentiment_subworkflow.context.is_some(), "SentimentAnalysis should have a context");
    assert_eq!(sentiment_subworkflow.agents.len(), 1, "SentimentAnalysis should have 1 agent");
    
    // Verificar el segundo subworkflow (EntityExtraction)
    let entity_subworkflow = &program.subworkflows[1];
    assert_eq!(entity_subworkflow.name, "EntityExtraction");
    
    // Verificar input/output del segundo subworkflow
    if let Some(input) = &entity_subworkflow.input {
        assert_eq!(input.len(), 1, "EntityExtraction should have 1 input parameter");
        assert_eq!(input[0], "document");
    } else {
        panic!("EntityExtraction should have input parameters");
    }
    
    if let Some(output) = &entity_subworkflow.output {
        assert_eq!(output.len(), 2, "EntityExtraction should have 2 output parameters");
        assert_eq!(output[0], "entities");
        assert_eq!(output[1], "categories");
    } else {
        panic!("EntityExtraction should have output parameters");
    }
    
    assert_eq!(entity_subworkflow.agents.len(), 2, "EntityExtraction should have 2 agents");
}

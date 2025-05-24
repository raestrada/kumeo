use kumeo_compiler::ast::Context;
use kumeo_compiler::parse;

#[test]
fn test_context_parsing() {
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
    
    // Parsear el ejemplo
    let program = parse(context_example).expect("Failed to parse Kumeo workflows with context");
    
    // Verificar la cantidad de workflows
    assert_eq!(program.workflows.len(), 3, "Should parse 3 workflows");
    
    // Verificar el primer workflow con KnowledgeBase context
    let workflow1 = &program.workflows[0];
    assert_eq!(workflow1.name, "ContextTestWorkflow");
    
    if let Some(Context::KnowledgeBase(name, _)) = &workflow1.context {
        assert_eq!(name, "spanish-language");
    } else {
        panic!("First workflow should have KnowledgeBase context");
    }
    
    // Verificar el segundo workflow con Database context
    let workflow2 = &program.workflows[1];
    assert_eq!(workflow2.name, "DatabaseContextWorkflow");
    
    if let Some(Context::Database(db_type, conn_string)) = &workflow2.context {
        assert_eq!(db_type, "postgres");
        assert_eq!(conn_string, "postgresql://user:pass@localhost:5432/mydb");
    } else {
        panic!("Second workflow should have Database context");
    }
    
    // Verificar el tercer workflow con BayesianNetwork context
    let workflow3 = &program.workflows[2];
    assert_eq!(workflow3.name, "BayesNetContextWorkflow");
    
    if let Some(Context::BayesianNetwork(name, _)) = &workflow3.context {
        assert_eq!(name, "medical-diagnosis");
    } else {
        panic!("Third workflow should have BayesianNetwork context");
    }
}

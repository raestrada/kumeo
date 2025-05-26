use kumeo_compiler::ast::*;
use kumeo_compiler::parser::parse;

#[test]
fn test_parse_simple_workflow() {
    let input = r#"
    workflow TestWorkflow {
        source input {
            type = "nats"
            topic = "input-topic"
        }
        
        target output {
            type = "nats"
            topic = "output-topic"
        }
        
        agent llm_agent {
            type = "llm"
            model = "llama3"
            prompt = "Process {{data}}"
        }
    }
    "#;

    let program = parse(input).expect("Debería parsear el workflow correctamente");
    assert_eq!(program.workflows.len(), 1);
    
    let workflow = &program.workflows[0];
    assert_eq!(workflow.name, "TestWorkflow");
    
    // Verificar source
    assert!(workflow.source.is_some());
    if let Some(Source::NATS(topic, _)) = &workflow.source {
        assert_eq!(topic, "input-topic");
    } else {
        panic!("Tipo de source incorrecto");
    }
    
    // Verificar target
    assert!(workflow.target.is_some());
    if let Some(Target::NATS(topic, _)) = &workflow.target {
        assert_eq!(topic, "output-topic");
    } else {
        panic!("Tipo de target incorrecto");
    }
    
    // Verificar agentes
    assert_eq!(workflow.agents.len(), 1);
    let agent = &workflow.agents[0];
    assert_eq!(agent.id.as_ref().unwrap(), "llm_agent");
    assert_eq!(agent.agent_type, AgentType::LLM);
}

#[test]
fn test_workflow_with_preprocessors() {
    let input = r#"
    workflow WithPreprocessors {
        source input { type = "nats" topic = "in" }
        target output { type = "nats" topic = "out" }
        
        preprocessors [
            { 
                id = "prep1"
                type = "preprocessor"
                script = "clean_data.py"
            }
        ]
        
        agent main_agent {
            type = "llm"
            model = "llama3"
        }
    }
    "#;

    let program = parse(input).expect("Debería parsear workflow con preprocesadores");
    let workflow = &program.workflows[0];
    
    assert!(workflow.preprocessors.is_some());
    let preprocessors = workflow.preprocessors.as_ref().unwrap();
    assert_eq!(preprocessors.len(), 1);
    assert_eq!(preprocessors[0].id.as_ref().unwrap(), "prep1");
}

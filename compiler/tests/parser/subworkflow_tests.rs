use kumeo_compiler::ast::*;
use kumeo_compiler::parser::parse;

#[test]
fn test_parse_subworkflow() {
    let input = r#"
    subworkflow DataProcessor {
        agent step1 {
            type = "llm"
            model = "llama3"
            prompt = "Step 1: {{data}}"
        }
        
        agent step2 {
            type = "ml"
            model = "classifier"
        }
    }
    "#;

    let program = parse(input).expect("Debería parsear el subworkflow correctamente");
    assert_eq!(program.subworkflows.len(), 1);
    
    let subworkflow = &program.subworkflows[0];
    assert_eq!(subworkflow.name, "DataProcessor");
    assert_eq!(subworkflow.agents.len(), 2);
    
    // Verificar el primer agente
    assert_eq!(subworkflow.agents[0].id.as_ref().unwrap(), "step1");
    assert_eq!(subworkflow.agents[0].agent_type, AgentType::LLM);
    
    // Verificar el segundo agente
    assert_eq!(subworkflow.agents[1].id.as_ref().unwrap(), "step2");
    assert_eq!(subworkflow.agents[1].agent_type, AgentType::MLModel);
}

#[test]
fn test_workflow_with_subworkflow() {
    let input = r#"
    workflow MainWorkflow {
        source input { type = "nats" topic = "in" }
        target output { type = "nats" topic = "out" }
        
        use DataProcessor as processor
        
        agent final_step {
            type = "llm"
            model = "llama3"
            prompt = "Final step: {{data}}"
        }
    }
    
    subworkflow DataProcessor {
        agent step1 { type = "llm" model = "llama3" }
    }
    "#;

    let program = parse(input).expect("Debería parsear workflow con subworkflow");
    
    // Debería tener un workflow y un subworkflow
    assert_eq!(program.workflows.len(), 1);
    assert_eq!(program.subworkflows.len(), 1);
    
    let workflow = &program.workflows[0];
    assert_eq!(workflow.agents.len(), 1); // Solo el agente final_step
    
    // Verificar que el subworkflow se parseó correctamente
    let subworkflow = &program.subworkflows[0];
    assert_eq!(subworkflow.name, "DataProcessor");
    assert_eq!(subworkflow.agents.len(), 1);
}

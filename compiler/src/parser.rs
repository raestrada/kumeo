use crate::ast::*;
use crate::error::{KumeoError, Result};
use crate::lexer::Lexer;
use crate::semantic::SemanticAnalyzer;
use lalrpop_util::lalrpop_mod;
use tracing::{debug, error, info};

// Generar el parser a partir de la gramática LALRPOP
lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused)]
    pub kumeo
);

/// Parse Kumeo source code into an AST
pub fn parse(input: &str) -> Result<Workflow> {
    info!("Parsing Kumeo source code");
    
    // Crear el lexer
    let lexer = match Lexer::new(input) {
        Ok(lexer) => lexer,
        Err(e) => {
            error!("Failed to create lexer: {}", e);
            return Err(e);
        }
    };
    
    // Obtener el parser generado por LALRPOP
    let parser = kumeo::WorkflowParser::new();
    
    // Parsear el código fuente
    let workflow = match parser.parse(lexer) {
        Ok(workflow) => {
            debug!("Successfully parsed workflow: {}", workflow.name);
            workflow
        }
        Err(e) => {
            // Convertir el error de LALRPOP a un error de Kumeo
            let error_msg = format!("Parse error: {}", e);
            error!("{}", error_msg);
            return Err(KumeoError::ParserError(error_msg));
        }
    };
    
    // Realizar análisis semántico
    let mut analyzer = SemanticAnalyzer::new();
    if let Err(e) = analyzer.analyze_workflow(&workflow) {
        error!("Semantic error: {}", e);
        return Err(e);
    }
    
    debug!("Successfully validated workflow: {}", workflow.name);
    Ok(workflow)
}

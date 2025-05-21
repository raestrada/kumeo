use crate::ast::Program;
use crate::error::{KumeoError, Result};
use lalrpop_util::ParseError;

/// Parse Kumeo source code into an AST
pub fn parse(input: &str) -> Result<Program> {
    // Use the LALRPOP-generated parser directly with the string input
    // This bypasses the need for a custom lexer integration
    match crate::kumeo::ProgramParser::new().parse(input) {
        Ok(program) => Ok(program),
        Err(err) => {
            // Convert LALRPOP error to KumeoError
            match err {
                ParseError::InvalidToken { location } => {
                    Err(KumeoError::ParserError {
                        line: count_lines(input, location),
                        column: compute_column(input, location),
                        message: format!("Invalid token at position {}", location),
                    })
                },
                ParseError::UnrecognizedEof { location, expected } => {
                    Err(KumeoError::ParserError {
                        line: count_lines(input, location),
                        column: compute_column(input, location),
                        message: format!("Unexpected end of file, expected: {}", expected.join(", ")),
                    })
                },
                ParseError::UnrecognizedToken { token: (start, _, end), expected } => {
                    let token_text = &input[start..end];
                    Err(KumeoError::ParserError {
                        line: count_lines(input, start),
                        column: compute_column(input, start),
                        message: format!("Unexpected token '{}', expected: {}", token_text, expected.join(", ")),
                    })
                },
                ParseError::ExtraToken { token: (start, _, end) } => {
                    let token_text = &input[start..end];
                    Err(KumeoError::ParserError {
                        line: count_lines(input, start),
                        column: compute_column(input, start),
                        message: format!("Extra token '{}'", token_text),
                    })
                },
                ParseError::User { error } => {
                    Err(KumeoError::ParserError {
                        line: 0,
                        column: 0,
                        message: format!("Parser error: {}", error),
                    })
                },
            }
        }
    }
}

// Helper function to count lines up to a position
fn count_lines(input: &str, pos: usize) -> usize {
    input[..pos.min(input.len())].chars().filter(|&c| c == '\n').count() + 1
}

// Helper function to compute column number
fn compute_column(input: &str, pos: usize) -> usize {
    let safe_pos = pos.min(input.len());
    if let Some(line_start) = input[..safe_pos].rfind('\n') {
        safe_pos - line_start
    } else {
        safe_pos + 1 // 1-based indexing
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_workflow() {
        let input = r#"
            workflow SimpleFlow {
                source: NATS("input")
                target: NATS("output")
                
                agents: [
                    LLM(
                        id: "processor",
                        engine: "ollama/llama3"
                    )
                ]
            }
        "#;
        
        match parse(input) {
            Ok(program) => {
                assert_eq!(program.workflows.len(), 1);
                assert_eq!(program.workflows[0].name, "SimpleFlow");
            },
            Err(e) => panic!("Failed to parse: {}", e),
        }
    }
    
    #[test]
    fn test_workflow_with_prompt() {
        let input = r#"
            workflow SimpleFlow {
                source: NATS("input")
                target: NATS("output")
                
                agents: [
                    LLM(
                        id: "processor",
                        engine: "ollama/llama3",
                        prompt: "Process this: {{data}}"
                    )
                ]
            }
        "#;
        
        match parse(input) {
            Ok(program) => {
                assert_eq!(program.workflows.len(), 1);
                assert_eq!(program.workflows[0].name, "SimpleFlow");
            },
            Err(e) => panic!("Failed to parse: {}", e),
        }
    }
}

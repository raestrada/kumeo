use crate::ast::Program;
use crate::error::{KumeoError, Result};
use crate::lexer::{Lexer, Token, TokenWithContext};
use lalrpop_util::ParseError;
use tracing::{debug, info, error};

/// A lexer adapter to connect our Logos lexer with LALRPOP
pub struct LexerAdapter<'input> {
    // Sin usar por ahora, pero podría ser útil para funcionalidades futuras
    #[allow(dead_code)]
    lexer: Lexer<'input>,
    token_index: usize,
    tokens: Vec<TokenWithContext<'input>>,
}

impl<'input> LexerAdapter<'input> {
    pub fn new(input: &'input str) -> Result<Self> {
        let lexer = Lexer::new(input)?;
        let tokens = lexer.get_tokens().clone();
        debug!(token_count = tokens.len(), "Initialized lexer adapter");
        
        Ok(Self {
            lexer,
            token_index: 0,
            tokens,
        })
    }
}

impl<'input> Iterator for LexerAdapter<'input> {
    type Item = std::result::Result<(usize, Token<'input>, usize), String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.token_index >= self.tokens.len() {
            return None;
        }
        
        let token_with_context = &self.tokens[self.token_index];
        self.token_index += 1;
        
        let start = token_with_context.span.start;
        let token = token_with_context.token.clone();
        let end = token_with_context.span.end;
        
        Some(Ok((start, token, end)))
    }
}

/// Parse Kumeo source code into an AST
pub fn parse(input: &str) -> Result<Program> {
    info!("Parsing Kumeo source code");
    
    // Create our Logos-based lexer adapter
    let lexer = match LexerAdapter::new(input) {
        Ok(lexer) => lexer,
        Err(e) => {
            error!(error = ?e, "Failed to initialize lexer");
            return Err(e);
        }
    };
    
    // Use our lexer with the LALRPOP-generated parser
    match crate::kumeo::ProgramParser::new().parse(input, lexer) {
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

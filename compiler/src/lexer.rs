use logos::{Logos, Span};
use std::fmt;
use crate::error::{KumeoError, Result};
use serde::{Serialize, Deserialize};

/// Represents a location in the source code
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub line: usize,
    pub column: usize,
    pub absolute: usize,
}

impl Location {
    pub fn new(line: usize, column: usize, absolute: usize) -> Self {
        Self { line, column, absolute }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// Represents a span in the source code
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SourceSpan {
    pub start: Location,
    pub end: Location,
}

impl SourceSpan {
    pub fn new(start: Location, end: Location) -> Self {
        Self { start, end }
    }

    pub fn from_span(span: Span, source: &str) -> Self {
        let start = Location::new(
            source[..span.start].matches('\n').count() + 1,
            span.start - source[..span.start].rfind('\n').unwrap_or(0),
            span.start,
        );
        let end = Location::new(
            source[..span.end].matches('\n').count() + 1,
            span.end - source[..span.start].rfind('\n').unwrap_or(0),
            span.end,
        );
        Self { start, end }
    }
}

impl fmt::Display for SourceSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

/// Token types for the Kumeo language
#[derive(Logos, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum Token {
    // Keywords
    #[token("workflow")]
    Workflow,
    
    #[token("source")]
    Source,
    
    #[token("target")]
    Target,
    
    #[token("models")]
    Models,
    
    #[token("schemas")]
    Schemas,
    
    #[token("config")]
    Config,
    
    // Agent types
    #[token("DataProcessor")]
    DataProcessor,
    
    #[token("MLModel")]
    MLModel,
    
    #[token("LLM")]
    LLM,
    
    #[token("Router")]
    Router,
    
    #[token("DecisionMatrix")]
    DecisionMatrix,
    
    #[token("HumanReview")]
    HumanReview,
    
    // Common fields
    #[token("id")]
    Id,
    
    #[token("input")]
    Input,
    
    #[token("output")]
    Output,
    
    #[token("model")]
    Model,
    
    #[token("when")]
    When,
    
    #[token("timeout")]
    Timeout,
    
    #[token("retry")]
    Retry,
    
    #[token("fallback")]
    Fallback,
    
    #[token("provider")]
    Provider,
    
    #[token("prompt")]
    Prompt,
    
    #[token("context")]
    Context,
    
    #[token("rules")]
    Rules,
    
    #[token("on_failure")]
    OnFailure,
    
    // UI and deployment
    #[token("ui")]
    Ui,
    
    #[token("notifications")]
    Notifications,
    
    #[token("deployment")]
    Deployment,
    
    #[token("base_images")]
    BaseImages,
    
    #[token("resources")]
    Resources,
    
    #[token("storage")]
    Storage,
    
    #[token("scaling")]
    Scaling,
    
    #[token("monitor")]
    Monitor,
    
    #[token("metrics")]
    Metrics,
    
    #[token("dashboard")]
    Dashboard,
    
    #[token("alerts")]
    Alerts,
    
    // Literals
    #[regex(r#"[a-zA-Z_][a-zA-Z0-9_]*"#, |lex| lex.slice().to_string())]
    Ident(String),
    
    #[regex(r#"[0-9]+\.?[0-9]*"#, |lex| lex.slice().parse().ok())]
    Number(f64),
    
    #[token("true")]
    True,
    
    #[token("false")]
    False,
    
    #[token("null")]
    Null,
    
    // Operators
    #[token("=")]
    Assign,
    
    #[token("==")]
    Equal,
    
    #[token("!=")]
    NotEqual,
    
    #[token("<")]
    Less,
    
    #[token("<=")]
    LessEqual,
    
    #[token(">")]
    Greater,
    
    #[token(">=")]
    GreaterEqual,
    
    #[token("+")]
    Plus,
    
    #[token("-")]
    Minus,
    
    #[token("*")]
    Star,
    
    #[token("/")]
    Slash,
    
    #[token("%")]
    Percent,
    
    #[token("!")]
    Not,
    
    #[token("&&")]
    And,
    
    #[token("||")]
    Or,
    
    #[token(".")]
    Dot,
    
    #[token(":")]
    Colon,
    
    #[token(",")]
    Comma,
    
    #[token(";")]
    Semicolon,
    
    // Delimiters
    #[token("(")]
    LeftParen,
    
    #[token(")")]
    RightParen,
    
    #[token("{")]
    LeftBrace,
    
    #[token("}")]
    RightBrace,
    
    #[token("[")]
    LeftBracket,
    
    #[token("]")]
    RightBracket,
    
    // String literals
    #[regex(r#"""[^"""]*""" | "[^"]*" | '[^']*'"#, |lex| lex.slice().to_string())]
    StringLit(String),
    
    // Whitespace (skipped)
    #[regex(r"[ \t\n\r]+", logos::skip)]
    #[regex(r"//[^\n\r]*", logos::skip)]
    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
    Whitespace,
    
    // Error token for invalid input
    #[error]
    Error,
}

/// A token with its source location and context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenWithContext<'source> {
    pub token: Token,
    pub span: SourceSpan,
    #[serde(skip_serializing, skip_deserializing)]
    pub source: &'source str,
    #[serde(skip_serializing, skip_deserializing)]
    line_starts: Vec<usize>,
}

/// The lexer for Kumeo
#[derive(Debug)]
pub struct Lexer<'source> {
    tokens: Vec<TokenWithContext<'source>>,
    current: usize,
    pub source: &'source str,
    line_starts: Vec<usize>,
}

impl<'source> Lexer<'source> {
    /// Create a new lexer for the given source text
    pub fn new(source: &'source str) -> Result<Self> {
        let mut line_starts = vec![0];
        line_starts.extend(
            source.match_indices('\n')
                .map(|(i, _)| i + 1)
        );

        let mut lexer = Token::lexer(source);
        let mut tokens = Vec::new();
        let mut errors = Vec::new();

        while let Some(token_result) = lexer.next() {
            let span = SourceSpan::from_span(lexer.span(), source);
            
            match token_result {
                Ok(token) => {
                    tokens.push(TokenWithContext {
                        token,
                        span,
                        source,
                        line_starts: line_starts.clone(),
                    });
                }
                Err(_) => {
                    errors.push(format!(
                        "Invalid token at {}",
                        span.start
                    ));
                }
            }
        }


        if !errors.is_empty() {
            return Err(KumeoError::LexerError(errors.join("\n")));
        }

        Ok(Self {
            tokens,
            current: 0,
            source,
            line_starts,
        })
    }
    
    /// Get the next non-whitespace token
    pub fn next_token(&mut self) -> Option<&TokenWithContext<'source>> {
        while self.current < self.tokens.len() {
            let token = &self.tokens[self.current];
            self.current += 1;
            if !matches!(token.token, Token::Whitespace) {
                return Some(token);
            }
        }
        None
    }
    
    /// Peek at the next non-whitespace token without advancing
    pub fn peek_token(&self) -> Option<&TokenWithContext<'source>> {
        let mut next = self.current;
        while next < self.tokens.len() {
            let token = &self.tokens[next];
            if !matches!(token.token, Token::Whitespace) {
                return Some(token);
            }
            next += 1;
        }
        None
    }
    
    /// Reset the lexer to the beginning
    pub fn reset(&mut self) {
        self.current = 0;
    }
    
    /// Get all tokens
    pub fn get_tokens(&self) -> &[TokenWithContext<'source>] {
        &self.tokens
    }
}

/// Helper to convert a byte position to line and column numbers
fn line_column(pos: usize, line_starts: &[usize]) -> (usize, usize) {
    match line_starts.binary_search(&pos) {
        Ok(line) => (line + 1, 1),
        Err(0) => (1, pos + 1),
        Err(line) => {
            let line_start = line_starts[line - 1];
            (line, pos - line_start + 1)
        }
    }
}

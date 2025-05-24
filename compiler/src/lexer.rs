use logos::{Logos, Span};
use std::fmt;
use crate::error::{KumeoError, Result};
use tracing::{debug, info, trace};

/// Token types for the Kumeo language
#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token<'source> {
    // Keywords
    #[token("workflow")]
    Workflow,
    
    #[token("subworkflow")]
    Subworkflow,
    
    #[token("integration")]
    Integration,
    
    #[token("source")]
    Source,
    
    #[token("target")]
    Target,
    
    #[token("context")]
    Context,
    
    #[token("agents")]
    Agents,
    
    #[token("input")]
    Input,
    
    #[token("output")]
    Output,
    
    #[token("mapping")]
    Mapping,
    
    #[token("use")]
    Use,
    
    #[token("config")]
    Config,
    
    #[token("preprocessors")]
    Preprocessors,
    
    #[token("monitor")]
    Monitor,
    
    #[token("deployment")]
    Deployment,
    
    #[token("if")]
    If,
    
    #[token("else")]
    Else,
    
    #[token("for")]
    For,
    
    #[token("in")]
    In,
    
    #[token("match")]
    Match,
    
    #[token("when")]
    When,
    
    // Agent types
    #[token("LLM")]
    LLM,
    
    #[token("MLModel")]
    MLModel,
    
    #[token("BayesianNetwork")]
    BayesianNetwork,
    
    #[token("DecisionMatrix")]
    DecisionMatrix,
    
    #[token("Router")]
    Router,
    
    #[token("Aggregator")]
    Aggregator,
    
    #[token("RuleEngine")]
    RuleEngine,
    
    #[token("DataNormalizer")]
    DataNormalizer,
    
    #[token("MissingValueHandler")]
    MissingValueHandler,
    
    #[token("HumanInLoop")]
    HumanInLoop,
    
    #[token("Custom")]
    Custom,
    
    // Context types
    #[token("KnowledgeBase")]
    KnowledgeBase,
    
    #[token("Database")]
    Database,
    
    // Source and target types
    #[token("NATS")]
    NATS,
    
    #[token("HTTP")]
    HTTP,
    
    #[token("FILE")]
    FILE,
    
    // Literals
    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        // Quitar las comillas del string literal
        let s = lex.slice();
        &s[1..s.len()-1]
    })]
    StringLiteral(&'source str),
    
    // Using a simpler regex for triple string literals (non-greedy operators not supported)  
    #[regex(r#""""[^"]*""""#, logos::skip)]
    TripleStringLiteral,
    
    // NumberLiteral is our main number token with highest priority
    #[regex(r"[0-9]+(\.[0-9]+)?([eE][+-]?[0-9]+)?", |lex| lex.slice(), priority = 2)]
    NumberLiteral(&'source str),
    
    // These are kept for compatibility but won't match due to lower priority
    // and the logos::skip callback
    #[regex(r"[0-9]+\.[0-9]+([eE][+-]?[0-9]+)?", logos::skip, priority = 1)]
    FloatLiteral,
    
    #[regex(r"[0-9]+", logos::skip, priority = 0)]
    IntegerLiteral,
    
    #[token("true")]
    True,
    
    #[token("false")]
    False,
    
    #[token("null")]
    Null,
    
    // Identifiers
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice())]
    Identifier(&'source str),
    
    // Operators
    #[token("=")]
    Assign,
    
    #[token("==")]
    Equal,
    
    #[token("!=")]
    NotEqual,
    
    #[token("<")]
    LessThan,
    
    #[token(">")]
    GreaterThan,
    
    #[token("<=")]
    LessThanEqual,
    
    #[token(">=")]
    GreaterThanEqual,
    
    #[token("+")]
    Plus,
    
    #[token("-")]
    Minus,
    
    #[token("*")]
    Multiply,
    
    #[token("/")]
    Divide,
    
    #[token("%")]
    Modulo,
    
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
    
    // Skip whitespace and comments
    #[regex(r"[ \t\r\n]+", logos::skip)]
    #[regex(r"//[^\n]*", logos::skip)]
    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
    Whitespace,
    
    // Error for invalid tokens
    // Error token (no attribute needed in Logos 0.13+)
    Error,
}

impl<'source> fmt::Display for Token<'source> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Workflow => write!(f, "workflow"),
            Token::Subworkflow => write!(f, "subworkflow"),
            Token::Integration => write!(f, "integration"),
            Token::Source => write!(f, "source"),
            Token::Target => write!(f, "target"),
            Token::Context => write!(f, "context"),
            Token::Agents => write!(f, "agents"),
            Token::Input => write!(f, "input"),
            Token::Output => write!(f, "output"),
            Token::Mapping => write!(f, "mapping"),
            Token::Use => write!(f, "use"),
            Token::Config => write!(f, "config"),
            Token::Preprocessors => write!(f, "preprocessors"),
            Token::Monitor => write!(f, "monitor"),
            Token::Deployment => write!(f, "deployment"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::For => write!(f, "for"),
            Token::In => write!(f, "in"),
            Token::Match => write!(f, "match"),
            Token::When => write!(f, "when"),
            
            // Agent types
            Token::LLM => write!(f, "LLM"),
            Token::MLModel => write!(f, "MLModel"),
            Token::BayesianNetwork => write!(f, "BayesianNetwork"),
            Token::DecisionMatrix => write!(f, "DecisionMatrix"),
            Token::Router => write!(f, "Router"),
            Token::Aggregator => write!(f, "Aggregator"),
            Token::RuleEngine => write!(f, "RuleEngine"),
            Token::DataNormalizer => write!(f, "DataNormalizer"),
            Token::MissingValueHandler => write!(f, "MissingValueHandler"),
            Token::HumanInLoop => write!(f, "HumanInLoop"),
            Token::Custom => write!(f, "Custom"),
            
            // Context types
            Token::KnowledgeBase => write!(f, "KnowledgeBase"),
            Token::Database => write!(f, "Database"),
            
            // Source and target types
            Token::NATS => write!(f, "NATS"),
            Token::HTTP => write!(f, "HTTP"),
            Token::FILE => write!(f, "FILE"),
            
            // Literals with values
            Token::StringLiteral(s) => write!(f, "string literal: {}", s),
            Token::NumberLiteral(n) => write!(f, "number literal: {}", n),
            Token::TripleStringLiteral => write!(f, "triple string literal"),
            Token::IntegerLiteral => write!(f, "integer literal"),
            Token::FloatLiteral => write!(f, "float literal"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::Null => write!(f, "null"),
            Token::Identifier(name) => write!(f, "identifier: {}", name),
            
            // Operators
            Token::Assign => write!(f, "="),
            Token::Equal => write!(f, "=="),
            Token::NotEqual => write!(f, "!="),
            Token::LessThan => write!(f, "<"),
            Token::GreaterThan => write!(f, ">"),
            Token::LessThanEqual => write!(f, "<="),
            Token::GreaterThanEqual => write!(f, ">="),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Multiply => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::Modulo => write!(f, "%"),
            Token::Not => write!(f, "!"),
            Token::And => write!(f, "&&"),
            Token::Or => write!(f, "||"),
            Token::Dot => write!(f, "."),
            Token::Colon => write!(f, ":"),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            
            // Delimiters
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::LeftBracket => write!(f, "["),
            Token::RightBracket => write!(f, "]"),
            Token::Whitespace => write!(f, "whitespace"),
            Token::Error => write!(f, "error"),
        }
    }
}

/// A token with its span and source text
#[derive(Debug, Clone)]
pub struct TokenWithContext<'source> {
    pub token: Token<'source>,
    pub span: Span,
    pub text: String,
    pub line: usize,
    pub column: usize,
}

/// The lexer for Kumeo
pub struct Lexer<'source> {
    pub source: &'source str,
    token_stream: Vec<TokenWithContext<'source>>,
    position: usize,
}

impl<'source> Lexer<'source> {
    /// Create a new lexer for the given source text
    pub fn new(source: &'source str) -> Result<Self> {
        let mut logos_lexer = Token::lexer(source);
        let mut token_stream = Vec::new();
        let mut line_starts = vec![0];
        
        // Find all line starts for line/column calculation
        for (i, c) in source.char_indices() {
            if c == '\n' {
                line_starts.push(i + 1);
            }
        }
        
        info!("Starting lexical analysis");
        
        // Process all tokens
        while let Some(token_result) = logos_lexer.next() {
            match token_result {
                Ok(token) => {
                    // Whitespace is already skipped by Logos with logos::skip
                    
                    let span = logos_lexer.span();
                    let text = &source[span.clone()];
                    
                    // Calculate line and column
                    let start_pos = span.start;
                    let (line, column) = line_column(start_pos, &line_starts);
                    
                    trace!(token = ?token, line = line, column = column, "Token processed");
                    
                    token_stream.push(TokenWithContext {
                        token,
                        span,
                        text: text.to_string(),
                        line,
                        column,
                    });
                }
                Err(_) => {
                    let span = logos_lexer.span();
                    let text = &source[span.clone()];
                    let start_pos = span.start;
                    let (line, column) = line_column(start_pos, &line_starts);
                    
                    // Add an error token
                    token_stream.push(TokenWithContext {
                        token: Token::Error,
                        span,
                        text: text.to_string(),
                        line,
                        column,
                    });
                    
                    let err_msg = format!(
                        "Invalid token '{}' at line {}, column {}",
                        text, line, column
                    );
                    
                    debug!(text = %text, line = line, column = column, "Lexer error");
                    
                    return Err(KumeoError::LexerError(err_msg));
                }
            }
        }
        
        debug!(token_count = token_stream.len(), "Lexical analysis complete");
        
        Ok(Lexer {
            source,
            token_stream,
            position: 0,
        })
    }
    
    /// Get the next token
    pub fn next_token(&mut self) -> Option<&TokenWithContext<'source>> {
        if self.position < self.token_stream.len() {
            let token = &self.token_stream[self.position];
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }
    
    /// Peek at the next token without advancing
    pub fn peek_token(&self) -> Option<&TokenWithContext<'source>> {
        if self.position < self.token_stream.len() {
            Some(&self.token_stream[self.position])
        } else {
            None
        }
    }
    
    /// Reset the lexer to the beginning
    pub fn reset(&mut self) {
        self.position = 0;
    }
    
    /// Get all tokens
    pub fn get_tokens(&self) -> &Vec<TokenWithContext<'source>> {
        &self.token_stream
    }
}

/// Calculate line and column numbers from byte position
fn line_column(pos: usize, line_starts: &[usize]) -> (usize, usize) {
    match line_starts.binary_search(&pos) {
        Ok(line) => (line + 1, 1),
        Err(line) => {
            let line_start = line_starts[line - 1];
            (line, pos - line_start + 1)
        }
    }
}

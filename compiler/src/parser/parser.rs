//! Parser implementation for the Kumeo DSL.

use pest::Parser as _;
use pest_derive::Parser;

// Include the generated parser
include!(concat!(env!("OUT_DIR"), "/parser.rs"));

/// The Kumeo parser.
pub struct Parser;

impl Parser {
    /// Parse a Kumeo DSL input string into Pest pairs.
    pub fn parse(input: &str) -> Result<pest::iterators::Pairs<'_, Rule>, pest::error::Error<Rule>> {
        KumeoParser::parse(Rule::program, input)
    }
}

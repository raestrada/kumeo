//! Parser for the Kumeo DSL using Pest.

pub mod error;
pub mod parser;

use std::collections::HashMap;

use pest::iterators::Pair;

use crate::{
    ast::*,
    parser::parser::{Parser, Rule},
};

use self::error::{ParseError, ParseResult};

/// Parse a Kumeo DSL input string into an AST.
pub fn parse(input: &str) -> ParseResult<Program> {
    let pairs = Parser::parse(input)?;
    let mut program = Program::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::workflow => {
                program.workflows.push(parse_workflow(pair)?);
            }
            Rule::subworkflow => {
                program.subworkflows.push(parse_subworkflow(pair)?);
            }
            _ => {
                return Err(ParseError::generic(format!(
                    "Unexpected rule: {:?}",
                    pair.as_rule()
                )));
            }
        }
    }

    Ok(program)
}

fn parse_workflow(pair: Pair<Rule>) -> ParseResult<Workflow> {
    let mut workflow = Workflow {
        name: String::new(),
        source: None,
        target: None,
        context: None,
        preprocessors: None,
        agents: Vec::new(),
        monitor: None,
        deployment: None,
    };

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::ident => {
                workflow.name = pair.as_str().to_string();
            }
            Rule::data_source => {
                workflow.source = Some(parse_data_source(pair)?);
            }
            Rule::data_target => {
                workflow.target = Some(parse_data_target(pair)?);
            }
            Rule::agent => {
                workflow.agents.push(parse_agent(pair)?);
            }
            _ => {}
        }
    }

    Ok(workflow)
}

fn parse_subworkflow(pair: Pair<Rule>) -> ParseResult<Subworkflow> {
    let mut subworkflow = Subworkflow {
        name: String::new(),
        input: None,
        output: None,
        context: None,
        agents: Vec::new(),
    };

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::ident => {
                subworkflow.name = pair.as_str().to_string();
            }
            Rule::agent => {
                subworkflow.agents.push(parse_agent(pair)?);
            }
            _ => {}
        }
    }

    Ok(subworkflow)
}

fn parse_data_source(pair: Pair<Rule>) -> ParseResult<Source> {
    let mut inner = pair.into_inner();
    let source_type = inner
        .next()
        .ok_or_else(|| ParseError::generic("Expected source type"))?;

    match source_type.as_str() {
        "NATS" => {
            let topic = inner
                .next()
                .and_then(|p| p.into_inner().next())
                .map(|p| p.as_str().trim_matches('"').to_string())
                .ok_or_else(|| ParseError::generic("Expected NATS topic"))?;

            let options = inner.next().map(parse_object).transpose()?;
            // Convertir HashMap<String, Value> a HashMap<String, String>
            let options = options.map(|opts| {
                opts.into_iter()
                    .filter_map(|(k, v)| {
                        if let Value::String(s) = v {
                            Some((k, s))
                        } else {
                            None
                        }
                    })
                    .collect()
            });
            Ok(Source::NATS(topic, options))
        }
        _ => Err(ParseError::generic("Unsupported source type")),
    }
}

fn parse_data_target(pair: Pair<Rule>) -> ParseResult<Target> {
    let mut inner = pair.into_inner();
    let target_type = inner
        .next()
        .ok_or_else(|| ParseError::generic("Expected target type"))?;

    match target_type.as_str() {
        "NATS" => {
            let topic = inner
                .next()
                .and_then(|p| p.into_inner().next())
                .map(|p| p.as_str().trim_matches('"').to_string())
                .ok_or_else(|| ParseError::generic("Expected NATS topic"))?;

            let options = inner.next().map(parse_object).transpose()?;
            // Convertir HashMap<String, Value> a HashMap<String, String>
            let options = options.map(|opts| {
                opts.into_iter()
                    .filter_map(|(k, v)| {
                        if let Value::String(s) = v {
                            Some((k, s))
                        } else {
                            None
                        }
                    })
                    .collect()
            });
            Ok(Target::NATS(topic, options))
        }
        _ => Err(ParseError::generic("Unsupported target type")),
    }
}

fn parse_agent(pair: Pair<Rule>) -> ParseResult<Agent> {
    let mut inner = pair.into_inner();
    let agent_type = inner
        .next()
        .ok_or_else(|| ParseError::generic("Expected agent type"))?;

    let agent_type = match agent_type.as_str() {
        "LLM" => AgentType::LLM,
        "MLModel" => AgentType::MLModel,
        "DataProcessor" => AgentType::DataProcessor,
        "Router" => AgentType::Router,
        "DecisionMatrix" => AgentType::DecisionMatrix,
        "HumanReview" => AgentType::HumanReview,
        _ => return Err(ParseError::generic("Unknown agent type")),
    };

    let mut id = None;
    let mut config = Vec::new();

    for pair in inner {
        match pair.as_rule() {
            Rule::pair => {
                let mut pair_inner = pair.into_inner();
                let key = pair_inner
                    .next()
                    .ok_or_else(|| ParseError::generic("Expected key"))?
                    .as_str()
                    .to_string();
                let value = pair_inner
                    .next()
                    .ok_or_else(|| ParseError::generic("Expected value"))?;

                if key == "id" {
                    id = Some(
                        value
                            .as_str()
                            .trim_matches('"')
                            .to_string(),
                    );
                } else {
                    config.push(Argument::Named(key, parse_value(value)?));
                }
            }
            _ => {}
        }
    }

    Ok(Agent {
        id,
        agent_type,
        config,
    })
}

fn parse_value(pair: Pair<Rule>) -> ParseResult<Value> {
    match pair.as_rule() {
        Rule::string => {
            let s = pair.as_str().trim_matches('"').to_string();
            Ok(Value::String(s))
        }
        Rule::number => {
            let num = pair
                .as_str()
                .parse::<f64>()
                .map_err(|e| ParseError::generic(format!("Invalid number: {}", e)))?;
            Ok(Value::Number(num))
        }
        Rule::boolean => {
            let b = pair.as_str() == "true";
            Ok(Value::Boolean(b))
        }
        Rule::null => Ok(Value::Null),
        Rule::array => {
            let values = pair
                .into_inner()
                .map(parse_value)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Value::Array(values))
        }
        Rule::object => {
            let obj = parse_object(pair)?;
            Ok(Value::Object(obj))
        }
        _ => Err(ParseError::generic("Unexpected value type")),
    }
}

fn parse_object(pair: Pair<Rule>) -> ParseResult<HashMap<String, Value>> {
    let mut map = HashMap::new();

    for pair in pair.into_inner() {
        if pair.as_rule() == Rule::pair {
            let mut inner = pair.into_inner();
            let key = inner
                .next()
                .ok_or_else(|| ParseError::generic("Expected key"))?
                .as_str()
                .to_string();
            let value = inner
                .next()
                .ok_or_else(|| ParseError::generic("Expected value"))?;
            map.insert(key, parse_value(value)?);
        }
    }

    Ok(map)
}

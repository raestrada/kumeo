//! Abstract Syntax Tree (AST) for the Kumeo DSL.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Represents a Kumeo program, which is a collection of workflows and subworkflows.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    /// The workflows defined in the program.
    pub workflows: Vec<Workflow>,
    /// The subworkflows defined in the program.
    pub subworkflows: Vec<Subworkflow>,
}

impl Program {
    /// Create a new, empty program.
    pub fn new() -> Self {
        Self {
            workflows: Vec::new(),
            subworkflows: Vec::new(),
        }
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a workflow in the Kumeo DSL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    /// The name of the workflow.
    pub name: String,
    /// The data source for the workflow.
    pub source: Option<Source>,
    /// The data target for the workflow.
    pub target: Option<Target>,
    /// The context for the workflow.
    pub context: Option<Context>,
    /// The preprocessor agents for the workflow.
    pub preprocessors: Option<Vec<Agent>>,
    /// The agents in the workflow.
    pub agents: Vec<Agent>,
    /// Monitoring configuration for the workflow.
    pub monitor: Option<HashMap<String, String>>,
    /// Deployment configuration for the workflow.
    pub deployment: Option<Deployment>,
}

/// Represents a subworkflow in the Kumeo DSL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subworkflow {
    /// The name of the subworkflow.
    pub name: String,
    /// The input parameters for the subworkflow.
    pub input: Option<Vec<String>>,
    /// The output parameters for the subworkflow.
    pub output: Option<Vec<String>>,
    /// The context for the subworkflow.
    pub context: Option<Context>,
    /// The agents in the subworkflow.
    pub agents: Vec<Agent>,
}

/// Represents a data source in the Kumeo DSL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Source {
    /// A NATS message broker source.
    NATS(String, Option<HashMap<String, String>>),
}

/// Represents a data target in the Kumeo DSL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Target {
    /// A NATS message broker target.
    NATS(String, Option<HashMap<String, String>>),
}

/// Represents context for a workflow or subworkflow.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Context {
    /// Configuration values.
    pub config: HashMap<String, Value>,
    /// Model definitions.
    pub models: HashMap<String, Model>,
    /// Schema definitions.
    pub schemas: HashMap<String, Schema>,
}

/// Represents a model in the Kumeo DSL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    /// The type of the model.
    pub model_type: String,
    /// The path to the model.
    pub path: String,
    /// Additional configuration for the model.
    pub config: HashMap<String, Value>,
}

/// Represents a schema in the Kumeo DSL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    /// The fields in the schema.
    pub fields: HashMap<String, String>,
    /// Whether the schema is strict.
    pub strict: bool,
}

/// Represents an agent in the Kumeo DSL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    /// The ID of the agent.
    pub id: Option<String>,
    /// The type of the agent.
    pub agent_type: AgentType,
    /// The configuration for the agent.
    pub config: Vec<Argument>,
}

/// Represents the type of an agent.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AgentType {
    /// A large language model agent.
    LLM,
    /// A machine learning model agent.
    MLModel,
    /// A data processing agent.
    DataProcessor,
    /// A router agent for directing data flows.
    Router,
    /// A decision matrix for complex decision making.
    DecisionMatrix,
    /// A human review step in the workflow.
    HumanReview,
}

/// Represents an argument to an agent or function.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Argument {
    /// A named argument.
    Named(String, Value),
    /// A positional argument.
    Positional(Value),
}

/// Represents a value in the Kumeo DSL.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Value {
    /// A string value.
    String(String),
    /// A number value.
    Number(f64),
    /// A boolean value.
    Boolean(bool),
    /// A null value.
    Null,
    /// An array of values.
    Array(Vec<Value>),
    /// A map of strings to values.
    Object(HashMap<String, Value>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Number(n) => write!(f, "{}", n),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
            Value::Array(arr) => {
                write!(f, "[")?;
                for (i, val) in arr.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", val)?;
                }
                write!(f, "]")
            }
            Value::Object(obj) => {
                write!(f, "{{")?;
                for (i, (k, v)) in obj.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "\"{}\": {}", k, v)?;
                }
                write!(f, "}}")
            }
        }
    }
}

/// Represents a deployment configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deployment {
    /// The name of the deployment.
    pub name: String,
    /// The namespace for the deployment.
    pub namespace: Option<String>,
    /// The replicas for the deployment.
    pub replicas: Option<u32>,
    /// The resources for the deployment.
    pub resources: Option<ResourceRequirements>,
    /// The environment variables for the deployment.
    pub env: Option<HashMap<String, String>>,
}

/// Represents resource requirements for a deployment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    /// The CPU requirements.
    pub cpu: Option<String>,
    /// The memory requirements.
    pub memory: Option<String>,
    /// The GPU requirements.
    pub gpu: Option<String>,
}

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Root node for a Kumeo program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub workflows: Vec<Workflow>,
    pub subworkflows: Vec<Subworkflow>,
    pub integrations: Vec<Integration>,
}

/// Workflow definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub name: String,
    pub source: Option<Source>,
    pub target: Option<Target>,
    pub context: Option<Context>,
    pub preprocessors: Option<Vec<Agent>>,
    pub agents: Vec<Agent>,
    pub monitor: Option<HashMap<String, Value>>,
    pub deployment: Option<HashMap<String, Value>>,
}

/// Subworkflow definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subworkflow {
    pub name: String,
    pub input: Option<Vec<String>>,
    pub output: Option<Vec<String>>,
    pub context: Option<Context>,
    pub agents: Vec<Agent>,
}

/// Integration between workflows and subworkflows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Integration {
    pub workflow: String,
    pub subworkflow: String,
    pub mapping: Mapping,
}

/// Mapping for integrations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mapping {
    pub input: HashMap<String, PathExpr>,
    pub output: HashMap<String, PathExpr>,
}

/// Event source definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Source {
    NATS(String, Option<HashMap<String, Value>>),
    HTTP(String, Option<HashMap<String, Value>>),
    Kafka(String, Option<HashMap<String, Value>>),
    MQTT(String, Option<HashMap<String, Value>>),
    Timer(String),
    File(String, Option<HashMap<String, Value>>),
    Custom(String, Vec<Value>),
}

/// Event target definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Target {
    NATS(String, Option<HashMap<String, Value>>),
    HTTP(String, Option<HashMap<String, Value>>),
    Kafka(String, Option<HashMap<String, Value>>),
    MQTT(String, Option<HashMap<String, Value>>),
    File(String, Option<HashMap<String, Value>>),
    Custom(String, Vec<Value>),
}

/// Context definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Context {
    KnowledgeBase(String, Option<HashMap<String, Value>>),
    BayesianNetwork(String, Option<HashMap<String, Value>>),
    Database(String, String),
    Custom(String, Vec<Value>),
}

/// Agent definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: Option<String>,
    pub agent_type: AgentType,
    pub config: Vec<Argument>,
}

/// Agent type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentType {
    LLM,
    MLModel,
    BayesianNetwork,
    DecisionMatrix,
    HumanInLoop,
    Router,
    Aggregator,
    RuleEngine,
    DataNormalizer,
    MissingValueHandler,
    Custom(String),
}

/// Argument for agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Argument {
    Positional(Value),
    Named(String, Value),
}

/// Path expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathExpr {
    pub components: Vec<String>,
}

/// Value types in Kumeo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
    Path(PathExpr),
}

impl Program {
    pub fn new() -> Self {
        Program {
            workflows: Vec::new(),
            subworkflows: Vec::new(),
            integrations: Vec::new(),
        }
    }
}

impl PathExpr {
    pub fn from_string(path: &str) -> Self {
        PathExpr {
            components: path.split('.').map(|s| s.to_string()).collect(),
        }
    }
    
    pub fn to_string(&self) -> String {
        self.components.join(".")
    }
}

use std::collections::HashMap;
use std::fmt;
use serde::{Serialize, Deserialize};

/// Common configuration for all agents
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentConfig {
    pub id: Option<String>,
    pub input: Option<String>,
    pub output: Option<String>,
    pub model: Option<String>,
    pub when: Option<String>,
    pub timeout: Option<String>,
    pub retry: Option<RetryPolicy>,
    pub fallback: Option<FallbackConfig>,
    #[serde(flatten)]
    pub custom: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub backoff: String, // e.g., "1s", "2s,5s,10s"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FallbackConfig {
    pub action: String, // "continue", "fail", "use_default"
    pub default: Option<Value>,
}

/// Root node for a Kumeo program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub workflows: Vec<Workflow>,
    pub subworkflows: Vec<Subworkflow>,
    pub integrations: Vec<Integration>,
}

/// Workflow definition with resources and agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub name: String,
    pub description: Option<String>,
    
    // Communication
    pub nats: Option<String>,
    
    // Data sources
    pub source: Option<Source>,
    
    // Output channels
    pub target: Option<Vec<Target>>,
    
    // Resource definitions
    pub models: Option<HashMap<String, ModelDef>>,
    pub schemas: Option<HashMap<String, String>>,
    pub config: Option<HashMap<String, Value>>,
    
    // Agent pipeline
    pub agents: Vec<Agent>,
    
    // Monitoring
    pub monitor: Option<MonitorConfig>,
    
    // Deployment configuration
    pub deployment: Option<DeploymentConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelDef {
    pub file: String,
    pub r#type: String,  // "onnx", "pytorch", etc.
    pub version: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorConfig {
    pub metrics: Vec<String>,
    pub dashboard: Option<String>,
    pub alerts: Option<Vec<AlertConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConfig {
    pub name: String,
    pub condition: String,
    pub severity: String,  // "info", "warning", "critical"
    pub actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentConfig {
    pub base_images: Option<HashMap<String, String>>,
    pub resources: Option<HashMap<String, ResourceConfig>>,
    pub storage: Option<HashMap<String, String>>,
    pub scaling: Option<ScalingConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConfig {
    pub cpu: String,
    pub memory: String,
    pub gpu: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingConfig {
    pub min_replicas: u32,
    pub max_replicas: u32,
    pub target_cpu: Option<u32>,
    pub target_memory: Option<u32>,
}
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

/// Agent definition with common configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    #[serde(flatten)]
    pub config: AgentConfig,
    #[serde(flatten)]
    pub agent_type: AgentType,
}

/// Core Agent Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentType {
    /// Data processing and transformation
    DataProcessor {
        config: HashMap<String, Value>,
    },
    
    /// Machine learning model execution
    MLModel {
        model: String,
        output_schema: Option<String>,
    },
    
    /// Large language model interface
    LLM {
        provider: LLMProvider,
        prompt: String,
        context: Option<Value>,
    },
    
    /// Conditional message routing
    Router {
        rules: HashMap<String, String>,
    },
    
    /// Validation against rules
    DecisionMatrix {
        rules: Vec<ValidationRule>,
        on_failure: Option<FailureAction>,
    },
    
    /// Human review workflow
    HumanReview {
        config: HumanReviewConfig,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LLMProvider {
    Ollama { model: String },
    OpenAI { model: String, api_key: Option<String> },
    // Add other providers as needed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub name: String,
    pub condition: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FailureAction {
    Quarantine { retry: Option<u32> },
    Reject,
    LogAndContinue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanReviewConfig {
    pub ui: HashMap<String, Value>,
    pub notifications: HashMap<String, Value>,
    pub timeout: Option<String>,
}
    DataNormalizer,
    MissingValueHandler,
    Custom(String),
}

impl fmt::Display for AgentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgentType::LLM => write!(f, "llm"),
            AgentType::MLModel => write!(f, "mlmodel"),
            AgentType::BayesianNetwork => write!(f, "bayesian-network"),
            AgentType::DecisionMatrix => write!(f, "decision-matrix"),
            AgentType::HumanInLoop => write!(f, "human-in-loop"),
            AgentType::Router => write!(f, "router"),
            AgentType::Aggregator => write!(f, "aggregator"),
            AgentType::RuleEngine => write!(f, "rule-engine"),
            AgentType::DataNormalizer => write!(f, "data-normalizer"),
            AgentType::MissingValueHandler => write!(f, "missing-value-handler"),
            AgentType::Custom(s) => write!(f, "{}", s),
        }
    }
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

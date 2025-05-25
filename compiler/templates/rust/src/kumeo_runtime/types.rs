use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Message represents an event message in the Kumeo system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Unique identifier for the message
    pub id: String,
    
    /// The topic or channel the message was published to
    pub topic: String,
    
    /// Message payload as a JSON value
    pub payload: serde_json::Value,
    
    /// Timestamp when the message was created
    pub timestamp: u64,
    
    /// Optional metadata associated with the message
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

/// Resource type represents external data accessible to agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    /// Unique identifier for the resource
    pub id: String,
    
    /// Content type of the resource
    pub content_type: String,
    
    /// Resource data
    pub data: Vec<u8>,
    
    /// Optional metadata associated with the resource
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

/// Subscription information
#[derive(Debug, Clone)]
pub struct Subscription {
    /// The topic being subscribed to
    pub topic: String,
    
    /// Unique subscription identifier
    pub id: String,
}

/// Agent state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AgentState {
    /// Agent is starting up
    Starting,
    
    /// Agent is running normally
    Running,
    
    /// Agent is stopping
    Stopping,
    
    /// Agent has stopped
    Stopped,
    
    /// Agent has encountered an error
    Error,
}

/// Agent metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetadata {
    /// Unique identifier for the agent
    pub id: String,
    
    /// Display name of the agent
    pub name: String,
    
    /// Type of agent (e.g., "llm", "ml", "bayesian")
    pub agent_type: String,
    
    /// Current state of the agent
    pub state: AgentState,
    
    /// Additional properties of the agent
    #[serde(default)]
    pub properties: HashMap<String, serde_json::Value>,
}

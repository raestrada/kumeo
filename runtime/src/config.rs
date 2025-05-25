//! Configuration for the Kumeo runtime

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuración de recursos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcesConfig {
    /// Base directory for local resources
    pub base_dir: PathBuf,
    /// Maximum cache time for resources (in seconds)
    pub cache_ttl: Option<u64>,
}

/// Configuración de mensajería
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagingConfig {
    /// NATS server URL
    pub nats_url: String,
    /// Prefix for messaging channels
    pub channel_prefix: Option<String>,
    /// Timeout for messaging operations (in seconds)
    pub timeout: Option<u64>,
}

/// Main runtime configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    /// Path to the UNIX socket for communication
    pub socket_path: PathBuf,
    
    /// Resource configuration
    pub resources: ResourcesConfig,
    
    /// Messaging configuration (optional)
    pub messaging: Option<MessagingConfig>,
    
    /// Logging level (e.g., "info", "debug", "trace")
    #[serde(default = "default_log_level")]
    pub log_level: String,
}

fn default_log_level() -> String {
    "info".to_string()
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        let socket_path = std::env::temp_dir().join("kumeo-runtime.sock");
        
        Self {
            socket_path,
            resources: ResourcesConfig {
                base_dir: std::env::current_dir().unwrap_or_default(),
                cache_ttl: Some(300), // 5 minutos por defecto
            },
            messaging: None,
            log_level: default_log_level(),
        }
    }
}

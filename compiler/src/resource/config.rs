//! Configuration for the resource management system

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::collections::HashMap;

/// Configuration for the resource manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConfig {
    /// The default strategy to use when no scheme is specified
    pub default_strategy: String,
    
    /// Base directory for local resources
    pub local: Option<LocalConfig>,
    
    /// S3-compatible storage configuration
    pub s3: Option<S3Config>,
    
    /// Git repository configuration
    pub git: Option<GitConfig>,
    
    /// HTTP/HTTPS configuration
    pub http: Option<HttpConfig>,
    
    /// Resource type mappings
    #[serde(default)]
    pub type_mappings: HashMap<String, String>,
}

impl Default for ResourceConfig {
    fn default() -> Self {
        Self {
            default_strategy: "local".to_string(),
            local: Some(LocalConfig::default()),
            s3: None,
            git: None,
            http: None,
            type_mappings: HashMap::new(),
        }
    }
}

/// Local filesystem configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalConfig {
    /// Base path for local resources
    pub path: PathBuf,
    
    /// Whether to create directories if they don't exist
    #[serde(default = "default_true")]
    pub create_dirs: bool,
}

impl Default for LocalConfig {
    fn default() -> Self {
        Self {
            path: PathBuf::from("resources"),
            create_dirs: true,
        }
    }
}

/// S3-compatible storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3Config {
    /// S3 endpoint URL
    pub endpoint: String,
    
    /// S3 bucket name
    pub bucket: String,
    
    /// S3 region
    pub region: String,
    
    /// Base path within the bucket
    #[serde(default)]
    pub base_path: Option<String>,
    
    /// Access key ID (can be set via env var)
    pub access_key: Option<String>,
    
    /// Secret access key (can be set via env var)
    pub secret_key: Option<String>,
    
    /// Session token (for temporary credentials)
    pub session_token: Option<String>,
}

/// Git repository configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitConfig {
    /// Git repository URL
    pub repo: String,
    
    /// Branch, tag, or commit hash
    #[serde(default = "default_branch")]
    pub reference: String,
    
    /// Base path within the repository
    #[serde(default)]
    pub base_path: Option<String>,
    
    /// Authentication token (if needed)
    pub auth_token: Option<String>,
    
    /// Whether to update the repository on startup
    #[serde(default = "default_true")]
    pub update_on_start: bool,
}

/// HTTP/HTTPS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpConfig {
    /// Base URL for HTTP resources
    pub base_url: String,
    
    /// Authentication token (if needed)
    pub auth_token: Option<String>,
    
    /// Additional headers to include in requests
    #[serde(default)]
    pub headers: HashMap<String, String>,
    
    /// Timeout in seconds
    #[serde(default = "default_http_timeout")]
    pub timeout_secs: u64,
    
    /// Whether to cache responses
    #[serde(default = "default_true")]
    pub cache_enabled: bool,
}

fn default_branch() -> String {
    "main".to_string()
}

fn default_true() -> bool {
    true
}

fn default_http_timeout() -> u64 {
    30
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            base_url: String::new(),
            auth_token: None,
            headers: HashMap::new(),
            timeout_secs: 30,
            cache_enabled: true,
        }
    }
}

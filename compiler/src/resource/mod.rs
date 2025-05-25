//! Resource management system for Kumeo
//! 
//! This module provides a unified interface for loading resources from various sources
//! including local filesystem, HTTP/HTTPS, S3-compatible storage, and Git repositories.

mod config;
mod error;
mod local;
mod http;
mod s3;
mod git;
mod manager;

use std::path::PathBuf;
use async_trait::async_trait;
pub use config::ResourceConfig;
pub use error::ResourceError;
pub use manager::ResourceManager;

/// Supported resource types
#[derive(Debug, Clone, PartialEq)]
pub enum ResourceType {
    KnowledgeBase,
    BayesianNetwork,
    Database,
    MLModel,
    Config,
    Other(String),
}

/// Trait that all resource loaders must implement
#[async_trait]
pub trait ResourceLoader: Send + Sync + std::fmt::Debug {
    /// Load a resource from the given URI
    async fn load(&self, uri: &str) -> Result<Vec<u8>, ResourceError>;
    
    /// Check if this loader can handle the given URI scheme
    fn supports_scheme(&self, scheme: &str) -> bool;
}

/// Represents a resource that can be loaded from various sources
#[derive(Debug)]
pub struct Resource {
    /// The raw content of the resource
    pub content: Vec<u8>,
    /// The source URI of the resource
    pub uri: String,
    /// The type of the resource
    pub resource_type: ResourceType,
    /// Optional content type if known
    pub content_type: Option<String>,
}

impl Resource {
    /// Create a new resource
    pub fn new(content: Vec<u8>, uri: String, resource_type: ResourceType) -> Self {
        Self {
            content,
            uri,
            resource_type,
            content_type: None,
        }
    }
    
    /// Get the content as a string if it's valid UTF-8
    pub fn as_str(&self) -> Result<&str, ResourceError> {
        std::str::from_utf8(&self.content).map_err(ResourceError::InvalidUtf8)
    }
    
    /// Get the content as a path if it's a local file
    pub fn as_local_path(&self) -> Option<PathBuf> {
        if self.uri.starts_with("file://") {
            Some(PathBuf::from(self.uri.trim_start_matches("file://")))
        } else {
            None
        }
    }
}

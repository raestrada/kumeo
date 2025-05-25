use super::{
    config::ResourceConfig, error::ResourceError, local::LocalLoader, Resource, ResourceLoader,
    ResourceType,
};
use async_trait::async_trait;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Manages loading of resources from various sources
#[derive(Debug, Clone)]
pub struct ResourceManager {
    loaders: Arc<RwLock<HashMap<String, Arc<dyn ResourceLoader + Send + Sync>>>>,
    config: ResourceConfig,
}

impl ResourceManager {
    /// Create a new ResourceManager with default configuration
    pub fn new() -> Self {
        let config = ResourceConfig::default();
        let mut manager = Self {
            loaders: Arc::new(RwLock::new(HashMap::new())),
            config,
        };
        
        // Initialize with default loaders
        manager.initialize_default_loaders();
        manager
    }

    /// Create a ResourceManager with custom configuration
    pub fn with_config(config: ResourceConfig) -> Self {
        let mut manager = Self {
            loaders: Arc::new(RwLock::new(HashMap::new())),
            config,
        };
        
        manager.initialize_default_loaders();
        manager
    }

    fn initialize_default_loaders(&mut self) {
        // Initialize local loader
        if let Some(local_config) = &self.config.local {
            let loader = LocalLoader::new(&local_config.path, local_config.create_dirs);
            self.register_loader("file".to_string(), Arc::new(loader));
        }

        // Initialize HTTP loader if configured
        if let Some(http_config) = &self.config.http {
            let loader = super::http::HttpLoader::new(
                &http_config.base_url,
                Some(http_config.headers.clone()),
                http_config.timeout_secs,
            );
            self.register_loader("http".to_string(), Arc::new(loader));
            self.register_loader("https".to_string(), Arc::new(loader));
        }

        // Note: S3 and Git loaders would be initialized here if their features are enabled
    }

    /// Register a custom loader for a specific scheme
    pub fn register_loader(&mut self, scheme: String, loader: Arc<dyn ResourceLoader + Send + Sync>) {
        let mut loaders = self.loaders.blocking_write();
        loaders.insert(scheme, loader);
    }

    /// Load a resource from a URI
    pub async fn load(&self, uri: &str) -> Result<Resource, ResourceError> {
        let (scheme, path) = self.parse_uri(uri)?;
        let loaders = self.loaders.read().await;
        
        if let Some(loader) = loaders.get(&scheme) {
            let content = loader.load(uri).await?;
            let resource_type = self.determine_resource_type(&path);
            
            Ok(Resource::new(content, uri.to_string(), resource_type))
        } else {
            Err(ResourceError::UnsupportedScheme(scheme))
        }
    }

    /// Parse a URI into scheme and path
    fn parse_uri(&self, uri: &str) -> Result<(String, String), ResourceError> {
        if uri.is_empty() {
            return Err(ResourceError::InvalidUri("Empty URI".to_string()));
        }

        // If no scheme is specified, use the default strategy
        if !uri.contains("://") {
            return Ok((self.config.default_strategy.clone(), uri.to_string()));
        }

        let parts: Vec<&str> = uri.splitn(2, "://").collect();
        if parts.len() != 2 {
            return Err(ResourceError::InvalidUri(uri.to_string()));
        }

        Ok((parts[0].to_string(), parts[1].to_string()))
    }

    /// Determine the resource type based on the file extension
    fn determine_resource_type(&self, path: &str) -> ResourceType {
        let path = PathBuf::from(path);
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            match ext.to_lowercase().as_str() {
                "kb" => ResourceType::KnowledgeBase,
                "bn" => ResourceType::BayesianNetwork,
                "db" | "sqlite" | "sqlite3" => ResourceType::Database,
                "onnx" | "pt" | "h5" | "pkl" => ResourceType::MLModel,
                "json" | "yaml" | "yml" | "toml" => ResourceType::Config,
                _ => ResourceType::Other(ext.to_string()),
            }
        } else {
            ResourceType::Other("unknown".to_string())
        }
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use tokio::fs;

    #[tokio::test]
    async fn test_local_loader() {
        let temp_dir = tempdir().unwrap();
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "test content").await.unwrap();

        let loader = LocalLoader::new(temp_dir.path(), false);
        let content = loader.load(&format!("file://{}", test_file.display()))
            .await
            .unwrap();

        assert_eq!(content, b"test content");
    }

    #[tokio::test]
    async fn test_resource_manager() {
        let temp_dir = tempdir().unwrap();
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "test content").await.unwrap();

        let config = ResourceConfig {
            local: Some(LocalConfig {
                path: temp_dir.path().to_path_buf(),
                create_dirs: false,
            }),
            ..Default::default()
        };

        let manager = ResourceManager::with_config(config);
        let resource = manager.load(&format!("file://{}", test_file.display()))
            .await
            .unwrap();

        assert_eq!(resource.content, b"test content");
        assert_eq!(resource.uri, format!("file://{}", test_file.display()));
    }
}
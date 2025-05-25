//! Resource management in the runtime

use crate::error::{Result, RuntimeError};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{SystemTime, Duration};
use url::Url;

/// Resource manager
#[derive(Debug, Clone)]
pub struct Manager {
    base_dir: PathBuf,
    cache: Arc<RwLock<HashMap<String, (Vec<u8>, SystemTime)>>>,
    cache_ttl: Option<Duration>,
}

impl Manager {
    /// Creates a new resource manager
    pub fn new(config: &super::super::config::ResourcesConfig) -> Result<Self> {
        let base_dir = config.base_dir.canonicalize()
            .map_err(|_| RuntimeError::Config(format!("Invalid base directory: {:?}", config.base_dir)))?;
            
        let cache_ttl = config.cache_ttl.map(Duration::from_secs);
            
        Ok(Self {
            base_dir,
            cache: Arc::new(RwLock::new(HashMap::new())),
            cache_ttl,
        })
    }
    
    /// Gets a resource
    pub async fn get(&self, uri: &str) -> Result<Vec<u8>> {
        // Check cache first
        if let Some((data, timestamp)) = self.check_cache(uri).await? {
            return Ok(data);
        }
        
        // Parse the URI
        let url = Url::parse(uri)
            .map_err(|e| RuntimeError::Resource(format!("Invalid URI: {}", e)))?;
        
        // Handle different schemes
        let data = match url.scheme() {
            "file" => self.load_file(url.path()).await?,
            "http" | "https" => self.load_http(uri).await?,
            _ => return Err(RuntimeError::Resource(format!("Unsupported scheme: {}", url.scheme()))),
        };
        
        // Almacenar en caché
        self.update_cache(uri, data.clone()).await;
        
        Ok(data)
    }
    
    /// Saves a resource
    pub async fn put(&self, uri: &str, data: &[u8]) -> Result<()> {
        let url = Url::parse(uri)
            .map_err(|e| RuntimeError::Resource(format!("Invalid URI: {}", e)))?;
            
        match url.scheme() {
            "file" => self.save_file(url.path(), data).await,
            _ => Err(RuntimeError::Resource(format!("Unsupported scheme for writing: {}", url.scheme()))),
        }
    }
    
    // Helper methods
    async fn load_file(&self, path: &str) -> Result<Vec<u8>> {
        let full_path = self.base_dir.join(path.trim_start_matches('/'));
        tokio::fs::read(&full_path)
            .await
            .map_err(|e| RuntimeError::Io(e).into())
    }
    
    async fn save_file(&self, path: &str, data: &[u8]) -> Result<()> {
        let full_path = self.base_dir.join(path.trim_start_matches('/'));
        if let Some(parent) = full_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        
        tokio::fs::write(&full_path, data)
            .await
            .map_err(|e| RuntimeError::Io(e).into())
    }
    
    async fn load_http(&self, url: &str) -> Result<Vec<u8>> {
        let response = reqwest::get(url)
            .await
            .map_err(|e| RuntimeError::Resource(format!("HTTP request failed: {}", e)))?;
            
        if !response.status().is_success() {
            return Err(RuntimeError::Resource(format!("HTTP error: {}", response.status())));
        }
        
        response.bytes()
            .await
            .map(|b| b.to_vec())
            .map_err(|e| RuntimeError::Resource(format!("Failed to read response: {}", e)).into())
    }
    
    async fn check_cache(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let cache = self.cache.read().await;
        if let Some((data, timestamp)) = cache.get(key) {
            if let Some(ttl) = self.cache_ttl {
                if let Ok(elapsed) = timestamp.elapsed() {
                    if elapsed <= ttl {
                        return Ok(Some(data.clone()));
                    }
                }
            } else {
                return Ok(Some(data.clone()));
            }
        }
        Ok(None)
    }
    
    async fn update_cache(&self, key: &str, data: Vec<u8>) {
        let mut cache = self.cache.write().await;
        cache.insert(key.to_string(), (data, SystemTime::now()));
    }
}

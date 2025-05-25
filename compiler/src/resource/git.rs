use super::{Resource, ResourceError, ResourceLoader, ResourceType};
use async_trait::async_trait;
use git2::{Repository, RepositoryOpenFlags};
use std::path::Path;
use std::sync::Arc;
use tempfile::TempDir;

/// Loader for Git repositories
#[derive(Debug, Clone)]
pub struct GitLoader {
    repo_url: String,
    reference: String,
    base_path: Option<String>,
    temp_dir: Option<Arc<TempDir>>,
}

impl GitLoader {
    pub fn new(
        repo_url: &str,
        reference: Option<&str>,
        base_path: Option<&str>,
    ) -> Result<Self, ResourceError> {
        Ok(Self {
            repo_url: repo_url.to_string(),
            reference: reference.unwrap_or("main").to_string(),
            base_path: base_path.map(|s| s.to_string()),
            temp_dir: None,
        })
    }

    async fn ensure_cloned(&mut self) -> Result<(), ResourceError> {
        if self.temp_dir.is_none() {
            let temp_dir = tempfile::tempdir()?;
            let repo = Repository::clone(&self.repo_url, temp_dir.path())?;
            
            // Checkout the specified reference
            let (object, reference) = repo.revparse_ext(&self.reference)?;
            repo.checkout_tree(&object, None)?;
            
            // If it's a branch, set HEAD to it
            if let Some(reference) = reference {
                let reference_name = reference.name().ok_or_else(|| {
                    ResourceError::Git(format!("Invalid reference: {}", self.reference))
                })?;
                repo.set_head(reference_name)?;
            }
            
            self.temp_dir = Some(Arc::new(temp_dir));
        }
        Ok(())
    }

    fn resolve_path(&self, path: &str) -> std::path::PathBuf {
        let base_path = self.temp_dir.as_ref().unwrap().path();
        if let Some(prefix) = &self.base_path {
            base_path.join(prefix).join(path)
        } else {
            base_path.join(path)
        }
    }
}

#[async_trait]
impl ResourceLoader for GitLoader {
    async fn load(&self, uri: &str) -> Result<Vec<u8>, ResourceError> {
        let path = uri.trim_start_matches("git://");
        let full_path = self.resolve_path(path);
        
        tokio::fs::read(&full_path)
            .await
            .map_err(|e| ResourceError::Git(e.to_string()))
    }

    fn supports_scheme(&self, scheme: &str) -> bool {
        scheme == "git"
    }
}
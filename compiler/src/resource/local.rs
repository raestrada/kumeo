use super::{Resource, ResourceError, ResourceLoader, ResourceType};
use async_trait::async_trait;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs;

/// Loader for local filesystem resources
#[derive(Debug, Clone)]
pub struct LocalLoader {
    base_path: PathBuf,
    create_dirs: bool,
}

impl LocalLoader {
    pub fn new<P: AsRef<Path>>(base_path: P, create_dirs: bool) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
            create_dirs,
        }
    }

    fn resolve_path(&self, uri: &str) -> Result<PathBuf, ResourceError> {
        let path = uri.trim_start_matches("file://");
        let path = Path::new(path);
        
        if path.is_absolute() {
            Ok(path.to_path_buf())
        } else {
            Ok(self.base_path.join(path))
        }
    }

    async fn ensure_dir_exists(&self, path: &Path) -> Result<(), ResourceError> {
        if let Some(parent) = path.parent() {
            if !parent.exists() && self.create_dirs {
                fs::create_dir_all(parent).await?;
            }
        }
        Ok(())
    }
}

#[async_trait]
impl ResourceLoader for LocalLoader {
    async fn load(&self, uri: &str) -> Result<Vec<u8>, ResourceError> {
        let path = self.resolve_path(uri)?;
        fs::read(&path)
            .await
            .map_err(|e| ResourceError::Io(e))
    }

    fn supports_scheme(&self, scheme: &str) -> bool {
        scheme == "file" || scheme.is_empty()
    }
}
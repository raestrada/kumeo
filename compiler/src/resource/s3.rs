use super::{Resource, ResourceError, ResourceLoader, ResourceType};
use async_trait::async_trait;
use aws_sdk_s3::Client;
use std::sync::Arc;

/// Loader for S3-compatible storage
#[derive(Debug, Clone)]
pub struct S3Loader {
    client: Client,
    bucket: String,
    base_path: Option<String>,
}

impl S3Loader {
    pub async fn new(
        endpoint: &str,
        region: &str,
        bucket: &str,
        access_key: Option<&str>,
        secret_key: Option<&str>,
        base_path: Option<String>,
    ) -> Result<Self, ResourceError> {
        let config = aws_config::from_env()
            .endpoint_url(endpoint)
            .region(aws_sdk_s3::config::Region::new(region.to_string()))
            .load()
            .await;

        let client = Client::new(&config);

        Ok(Self {
            client,
            bucket: bucket.to_string(),
            base_path,
        })
    }

    fn resolve_key(&self, uri: &str) -> String {
        let key = uri.trim_start_matches("s3://");
        if let Some(base_path) = &self.base_path {
            format!("{}/{}", base_path.trim_end_matches('/'), key.trim_start_matches('/'))
        } else {
            key.to_string()
        }
    }
}

#[async_trait]
impl ResourceLoader for S3Loader {
    async fn load(&self, uri: &str) -> Result<Vec<u8>, ResourceError> {
        let key = self.resolve_key(uri);
        
        let response = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(&key)
            .send()
            .await?;

        let bytes = response
            .body
            .collect()
            .await
            .map_err(|e| ResourceError::S3(e.to_string()))?;

        Ok(bytes.to_vec())
    }

    fn supports_scheme(&self, scheme: &str) -> bool {
        scheme == "s3"
    }
}
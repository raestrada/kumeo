use super::{Resource, ResourceError, ResourceLoader, ResourceType};
use async_trait::async_trait;
use reqwest::header;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

/// Loader for HTTP/HTTPS resources
#[derive(Debug, Clone)]
pub struct HttpLoader {
    client: reqwest::Client,
    base_url: String,
    headers: HashMap<String, String>,
    timeout: Duration,
}

impl HttpLoader {
    pub fn new(
        base_url: impl Into<String>,
        headers: Option<HashMap<String, String>>,
        timeout_secs: u64,
    ) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: base_url.into(),
            headers: headers.unwrap_or_default(),
            timeout: Duration::from_secs(timeout_secs),
        }
    }

    fn build_url(&self, uri: &str) -> String {
        if uri.starts_with("http://") || uri.starts_with("https://") {
            uri.to_string()
        } else {
            format!("{}/{}", self.base_url.trim_end_matches('/'), uri.trim_start_matches('/'))
        }
    }
}

#[async_trait]
impl ResourceLoader for HttpLoader {
    async fn load(&self, uri: &str) -> Result<Vec<u8>, ResourceError> {
        let url = self.build_url(uri);
        let mut request = self.client.get(&url);

        for (key, value) in &self.headers {
            request = request.header(key, value);
        }

        let response = request
            .send()
            .await
            .map_err(|e| ResourceError::Http(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ResourceError::Http(format!(
                "HTTP error: {}",
                response.status()
            )));
        }

        response
            .bytes()
            .await
            .map(|b| b.to_vec())
            .map_err(|e| ResourceError::Http(e.to_string()))
    }

    fn supports_scheme(&self, scheme: &str) -> bool {
        scheme == "http" || scheme == "https"
    }
}
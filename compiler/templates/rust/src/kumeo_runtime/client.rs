use std::path::Path;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use futures::Stream;
use tokio::net::UnixStream;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::{Endpoint, Uri};
use tower::service_fn;
use serde_json::{json, Value};

use crate::kumeo_runtime::error::RuntimeError;
use crate::kumeo_runtime::types::{Message, Resource};

// Import the generated protobuf code
use kumeo_protos::runtime_service_client::RuntimeServiceClient;
use kumeo_protos::{
    PublishRequest, SubscribeRequest, SubscribeResponse, 
    ResourceRequest, ResourceResponse, AgentRequest, AgentResponse
};

/// Default socket path for the Kumeo Runtime
pub const DEFAULT_SOCKET_PATH: &str = "/run/kumeo/runtime.sock";

/// Client for interacting with the Kumeo Runtime
pub struct RuntimeClient {
    client: RuntimeServiceClient<tonic::transport::Channel>,
    agent_id: String,
}

impl RuntimeClient {
    /// Create a new client connected to the default Unix socket
    pub async fn new() -> Result<Self, RuntimeError> {
        let socket_path = std::env::var("KUMEO_RUNTIME_SOCKET")
            .unwrap_or_else(|_| DEFAULT_SOCKET_PATH.to_string());
        
        let agent_id = std::env::var("AGENT_ID")
            .unwrap_or_else(|_| uuid::Uuid::new_v4().to_string());
            
        Self::with_socket_path(Path::new(&socket_path), agent_id).await
    }
    
    /// Create a new client with a specific socket path and agent ID
    pub async fn with_socket_path(socket_path: &Path, agent_id: String) -> Result<Self, RuntimeError> {
        // Create a custom channel that connects to the Unix socket
        let channel = Endpoint::try_from("http://[::]:50051") // The URI will be ignored
            .map_err(|e| RuntimeError::ConnectionError(format!("Failed to create endpoint: {}", e)))?
            .connect_with_connector(service_fn(move |_: Uri| {
                let socket_path = socket_path.to_owned();
                async move {
                    UnixStream::connect(socket_path)
                        .await
                        .map_err(|e| {
                            std::io::Error::new(
                                std::io::ErrorKind::Other,
                                format!("Failed to connect to unix socket: {}", e),
                            )
                        })
                }
            }))
            .await
            .map_err(|e| RuntimeError::ConnectionError(format!("Failed to connect to runtime: {}", e)))?;

        Ok(Self {
            client: RuntimeServiceClient::new(channel),
            agent_id,
        })
    }

    /// Get the agent ID associated with this client
    pub fn agent_id(&self) -> &str {
        &self.agent_id
    }

    /// Publish a JSON message to a topic
    pub async fn publish_json(&mut self, topic: &str, payload: Value) -> Result<(), RuntimeError> {
        let json_bytes = serde_json::to_vec(&payload)
            .map_err(|e| RuntimeError::ProtocolError(format!("Failed to serialize JSON: {}", e)))?;
            
        self.publish(topic, &json_bytes).await
    }

    /// Publish a raw message to a topic
    pub async fn publish(&mut self, topic: &str, payload: &[u8]) -> Result<(), RuntimeError> {
        let request = tonic::Request::new(PublishRequest {
            topic: topic.to_string(),
            payload: payload.to_vec(),
            agent_id: self.agent_id.clone(),
            headers: Default::default(),
        });

        self.client
            .publish(request)
            .await
            .map_err(RuntimeError::from)?;

        Ok(())
    }

    /// Subscribe to a topic
    pub async fn subscribe(&mut self, topic: &str) -> Result<impl Stream<Item = Result<Message, RuntimeError>>, RuntimeError> {
        let (tx, rx) = mpsc::channel(32);
        let topic = topic.to_string();
        let agent_id = self.agent_id.clone();

        let mut client = self.client.clone();

        tokio::spawn(async move {
            let request = tonic::Request::new(SubscribeRequest {
                topic,
                agent_id,
                queue_group: None,
            });

            let mut stream = match client.subscribe(request).await {
                Ok(response) => response.into_inner(),
                Err(e) => {
                    let _ = tx
                        .send(Err(RuntimeError::from(e)))
                        .await;
                    return;
                }
            };

            while let Some(result) = stream.message().await.transpose() {
                match result {
                    Ok(response) => {
                        // Try to parse as JSON
                        match serde_json::from_slice::<Value>(&response.payload) {
                            Ok(json_value) => {
                                let message = Message {
                                    id: response.id,
                                    topic: response.topic,
                                    payload: json_value,
                                    timestamp: response.timestamp,
                                    metadata: response.headers,
                                };
                                
                                if tx.send(Ok(message)).await.is_err() {
                                    break;
                                }
                            },
                            Err(e) => {
                                let _ = tx
                                    .send(Err(RuntimeError::ProtocolError(
                                        format!("Failed to parse message as JSON: {}", e)
                                    )))
                                    .await;
                            }
                        }
                    }
                    Err(e) => {
                        let _ = tx
                            .send(Err(RuntimeError::from(e)))
                            .await;
                        break;
                    }
                }
            }
        });

        Ok(ReceiverStream::new(rx))
    }

    /// Get a resource from the runtime
    pub async fn get_resource(&mut self, uri: &str) -> Result<Resource, RuntimeError> {
        let request = tonic::Request::new(ResourceRequest {
            uri: uri.to_string(),
            agent_id: self.agent_id.clone(),
        });

        let response = self
            .client
            .get_resource(request)
            .await
            .map_err(RuntimeError::from)?
            .into_inner();
            
        Ok(Resource {
            id: response.id,
            content_type: response.content_type,
            data: response.data,
            metadata: response.metadata,
        })
    }
    
    /// Register an agent with the runtime
    pub async fn register_agent(&mut self, name: &str, agent_type: &str) -> Result<(), RuntimeError> {
        let request = tonic::Request::new(AgentRequest {
            id: self.agent_id.clone(),
            name: name.to_string(),
            agent_type: agent_type.to_string(),
            action: "register".to_string(),
            properties: Default::default(),
        });
        
        self.client
            .agent_action(request)
            .await
            .map_err(RuntimeError::from)?;
            
        Ok(())
    }
}

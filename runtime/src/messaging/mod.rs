//! Messaging handling in the runtime

use crate::error::{Result, RuntimeError};
use async_trait::async_trait;
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::mpsc;

/// Interface for message handling
#[async_trait]
pub trait MessageHandler: Send + Sync + 'static {
    /// Processes a received message
    async fn handle_message(&self, subject: &str, payload: &[u8], headers: Option<&HashMap<String, String>>) -> Result<()>;
}

/// Subscription configuration
pub struct SubscriptionConfig {
    /// Topic to subscribe to
    pub subject: String,
    /// Queue group (optional)
    pub queue_group: Option<String>,
    /// Subscription duration (None for indefinite)
    pub timeout: Option<Duration>,
}

/// Messaging handler with NATS support
pub struct Manager {
    client: Option<async_nats::Client>,
    config: crate::config::MessagingConfig,
}

impl Manager {
    /// Creates a new instance of the message handler
    pub async fn new(config: &crate::config::MessagingConfig) -> Result<Self> {
        #[cfg(feature = "nats")]
        {
            let client = async_nats::connect(&config.nats_url)
                .await
                .map_err(|e| RuntimeError::Messaging(format!("Failed to connect to NATS: {}", e)))?;
                
            Ok(Self {
                client: Some(client),
                config: config.clone(),
            })
        }
        
        #[cfg(not(feature = "nats"))]
        Err(RuntimeError::Messaging("NATS support not compiled in".into()))
    }
    
    /// Publishes a message
    pub async fn publish(&self, subject: &str, payload: &[u8], headers: Option<HashMap<String, String>>) -> Result<()> {
        #[cfg(feature = "nats")]
        {
            if let Some(client) = &self.client {
                let mut msg = client.publish(
                    format!("{}{}", self.config.channel_prefix.as_deref().unwrap_or(""), subject),
                    payload.to_vec().into()
                );
                
                if let Some(headers_map) = headers {
                    for (key, value) in headers_map {
                        msg = msg.header(&key, &value);
                    }
                }
                
                msg.await
                    .map_err(|e| RuntimeError::Messaging(format!("Failed to publish message: {}", e)))?;
                
                Ok(())
            } else {
                Err(RuntimeError::Messaging("NATS client not initialized".into()))
            }
        }
        
        #[cfg(not(feature = "nats"))]
        Err(RuntimeError::Messaging("NATS support not compiled in".into()))
    }
    
    /// Subscribes to a topic
    pub async fn subscribe<H: MessageHandler>(
        &self,
        config: SubscriptionConfig,
        handler: H,
    ) -> Result<()> {
        #[cfg(feature = "nats")]
        {
            let client = self.client.as_ref()
                .ok_or_else(|| RuntimeError::Messaging("NATS client not initialized".to_string()))?;
                
            let subject = format!(
                "{}{}", 
                self.config.channel_prefix.as_deref().unwrap_or(""), 
                config.subject
            );
            
            let mut subscription = if let Some(queue_group) = &config.queue_group {
                client.queue_subscribe(&subject, queue_group).await
            } else {
                client.subscribe(&subject).await
            }.map_err(|e| RuntimeError::Messaging(format!("Failed to subscribe: {}", e)))?;
            
            // Iniciar tarea para manejar mensajes
            let handler = std::sync::Arc::new(handler);
            
            tokio::spawn(async move {
                while let Some(message) = subscription.next().await {
                    let handler = handler.clone();
                    let subject = message.subject.clone();
                    let payload = message.payload.to_vec();
                    let headers = message.headers.clone();
                    
                    tokio::spawn(async move {
                        if let Err(e) = handler.handle_message(&subject, &payload, headers.as_ref()).await {
                            tracing::error!("Error handling message: {}", e);
                        }
                    });
                }
            });
            
            Ok(())
        }
        
        #[cfg(not(feature = "nats"))]
        Err(RuntimeError::Messaging("NATS support not compiled in".into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    
    struct TestHandler {
        received: Arc<Mutex<Vec<(String, Vec<u8>)>>>,
    }
    
    #[async_trait]
    impl MessageHandler for TestHandler {
        async fn handle_message(&self, subject: &str, payload: &[u8], _headers: Option<&HashMap<String, String>>) -> Result<()> {
            let mut received = self.received.lock().await;
            received.push((subject.to_string(), payload.to_vec()));
            Ok(())
        }
    }
    
    #[tokio::test]
    async fn test_messaging() {
        // Este test requiere un servidor NATS en localhost:4222
        // Se puede ejecutar con: docker run -p 4222:4222 nats:latest
        
        if let Ok(config) = crate::config::MessagingConfig::new("nats://localhost:4222".to_string()) {
            let manager = Manager::new(&config).await;
            if let Ok(manager) = manager {
                let received = Arc::new(Mutex::new(Vec::new()));
                let handler = TestHandler {
                    received: received.clone(),
                };
                
                // Suscribirse a un tema
                let sub_config = SubscriptionConfig {
                    subject: "test.subject".to_string(),
                    queue_group: None,
                    timeout: Some(Duration::from_secs(5)),
                };
                
                if manager.subscribe(sub_config, handler).await.is_ok() {
                    // Publicar un mensaje
                    let payload = b"test payload";
                    if manager.publish("test.subject", payload, None).await.is_ok() {
                        // Esperar un momento para que llegue el mensaje
                        tokio::time::sleep(Duration::from_millis(100)).await;
                        
                        // Verificar que se recibió el mensaje
                        let received = received.lock().await;
                        assert!(!received.is_empty());
                        assert_eq!(received[0].0, "test.subject");
                        assert_eq!(received[0].1, payload);
                    }
                }
            }
        }
    }
}

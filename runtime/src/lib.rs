//! Kumeo Runtime
//! 
//! This module provides a runtime environment for Kumeo agents,
//! handling resources, communication, and other low-level operations.

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

pub mod config;
pub mod error;
pub mod resources;
pub mod messaging;
pub mod server;

// Re-export of the most common types
pub use config::RuntimeConfig;
pub use error::{Result, RuntimeError};

/// Initializes the runtime with the provided configuration
pub async fn init(config: RuntimeConfig) -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(config.log_level.clone())
        .init();
    
    // Initialize resources
    let resource_manager = resources::Manager::new(&config.resources)?;
    
    // Initialize messaging if enabled
    let messaging = if let Some(messaging_config) = &config.messaging {
        Some(messaging::Manager::new(messaging_config).await?)
    } else {
        None
    };
    
    // Start the server
    let server = server::Server::new(config.socket_path, resource_manager, messaging);
    server.run().await?;
    
    Ok(())
}
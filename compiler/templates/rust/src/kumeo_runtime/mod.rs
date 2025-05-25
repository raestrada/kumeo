// Kumeo Runtime - Rust Client module
// This module provides integration with the Kumeo Runtime sidecar

mod client;
mod error;
mod types;

pub use client::RuntimeClient;
pub use error::RuntimeError;
pub use types::*;

// Re-export everything needed for agent implementations
pub mod prelude {
    pub use super::RuntimeClient;
    pub use super::error::RuntimeError;
    pub use super::types::*;
    
    // Common traits used by agents
    pub use async_trait::async_trait;
    pub use serde_json::{json, Value};
}

//! Módulo principal para las definiciones del AST de Kumeo.

pub mod types;

// Re-exportar los tipos principales para facilitar el acceso
pub use types::{
    Program, Workflow, Subworkflow, Source, Target, Context, Model, Schema, Agent, AgentType,
    Deployment, ResourceRequirements, Argument, Value
};

# Kumeo Architecture

## Overview

Kumeo is a declarative language and platform for orchestrating distributed agent workflows. The architecture is designed around a core principle: simplify the complex task of making heterogeneous agents collaborate while ensuring enterprise-grade performance and scalability.

This document outlines the architectural components and their interactions within the Kumeo ecosystem.

## System Architecture

![Kumeo Architecture Diagram](../branding/architecture_diagram.png)

### Core Components

1. **Kumeo DSL** - The declarative language used to define agent workflows
2. **Compiler** - Transforms DSL code into executable Rust
3. **Runtime** - Executes compiled workflows and manages agent communication
4. **Monitoring System** - Tracks performance, resource usage, and workflow status
5. **UI** - Visual editor and dashboard for workflow design and monitoring

### Data Flow

```
┌──────────┐     ┌──────────┐     ┌─────────────┐     ┌──────────────┐
│ Kumeo DSL│────▶│ Compiler │────▶│ Rust Code + │────▶│ Kubernetes   │
│ (.kumeo) │     │          │     │ K8s Manifests│     │ Deployment   │
└──────────┘     └──────────┘     └─────────────┘     └──────────────┘
                                                              │
                                                              ▼
┌──────────┐     ┌──────────┐     ┌─────────────┐     ┌──────────────┐
│ Monitoring│◀───│ NATS     │◀───│ Agent       │◀───│ Kumeo Runtime │
│ Dashboard │     │ Events   │     │ Interactions │     │ (Rust)       │
└──────────┘     └──────────┘     └─────────────┘     └──────────────┘
```

## Component Details

### 1. Kumeo DSL

The Kumeo Domain-Specific Language provides a declarative syntax for defining:

- **Workflow Structure** - The overall flow definition
- **Agents** - Individual processing units (LLMs, ML models, etc.)
- **Events** - Communication channels between agents
- **Context** - Shared data and knowledge bases
- **Deployment Configuration** - Resource requirements and scaling parameters

**Example**:
```kumeo
workflow FraudDetection {
  source: NATS("transactions")
  target: NATS("alerts")
  context: BayesianNetwork("risk.bn")

  agents: [
    LLM("ollama/llama3", prompt="Classify {{data}} as fraud? Context: {{context}}"),
    MLModel("isolation_forest.pkl", input=LLM.output),
    DecisionMatrix("policy.dmx", input=MLModel.output)
  ]
}
```

### 2. Compiler

The compiler is built in Rust and performs:

- **Lexical and Syntactic Analysis** - Parses .kumeo files into an Abstract Syntax Tree
- **Semantic Validation** - Ensures workflow integrity and agent compatibility
- **Multi-Language Code Generation** - Produces optimized code in the most appropriate language for each component
- **Kubernetes Manifest Generation** - Creates deployment configurations
- **Automated Deployment Process** - Handles the complete deployment lifecycle

The compiler enforces type safety and correct agent interaction patterns, catching errors at compile-time rather than runtime.

#### Multi-Language Code Generation

Kumeo uses a multi-backstage approach to code generation, selecting the most appropriate language for each agent type:

| Agent Type | Generated Language | Rationale |
|------------|-------------------|------------|
| LLM | Rust | Performance, native Ollama integration |
| Basic Transformations | Rust | Performance, memory safety |
| Routing & I/O | Rust | Low latency, NATS performance |
| ML Models | Python | Ecosystem compatibility (scikit-learn, TensorFlow) |
| Bayesian Networks | Python | Rich libraries (PyMC3, pomegranate) |
| Custom Logic | User's choice | Flexibility for specific requirements |

This approach enables Kumeo to leverage the strengths of each language:

- **Rust** for performance-critical components and system infrastructure
- **Python** for data science and machine learning capabilities
- **Inter-language communication** via NATS for seamless integration

#### Template-Based Generation

To minimize errors in generated code, Kumeo employs a template-based approach:

1. **Language-specific templates** for each agent type and function
2. **Parameter substitution** with validation to prevent injection issues
3. **Consistent interfaces** across languages to ensure compatibility
4. **Generated tests** to verify correct operation

### 3. Runtime

The Kumeo runtime is a multi-language execution environment with the following capabilities:

### Runtime Components

#### Core Runtime (Rust)
- **High-Performance Foundation** - Built in Rust for maximum performance
- **Agent Lifecycle Management** - Initializes, monitors, and terminates agents
- **Event Routing** - Efficient message passing between agents via NATS
- **Resilience** - Built-in retry logic and failure handling

#### Python Runtime
- **Rapid Development** - Easy agent development with Python
- **ML/Data Science** - Native support for ML frameworks (scikit-learn, PyTorch, etc.)
- **Async/Await** - First-class support for asynchronous operations
- **Type Hints** - Full type annotation support for better development experience

#### Language Interoperability
- **Transparent Communication** - Agents in different languages can communicate seamlessly
- **Unified API** - Consistent programming model across languages
- **Automatic Serialization** - Built-in serialization of common data types
- **Provides Observability** - Exposes metrics and logs for monitoring

The runtime is designed to be lightweight, with minimal overhead and maximum throughput. It communicates with the NATS messaging system for event distribution.

#### Agent Types and Integration

| Agent Type | Implementation | Integration Method |
|------------|---------------|-------------------|
| LLM | Rust API clients | Direct API calls (OpenAI, Ollama, etc.) |
| ML Model | ONNX Runtime | Direct inference with serialized models |
| Bayesian Network | Custom Rust implementation | In-memory inference |
| Human-in-the-loop | Web interface | Webhook notifications and UI integration |
| Custom | Plugin system | Dynamic loading via WebAssembly modules |

### 4. NATS Integration

NATS serves as the messaging backbone of Kumeo:

- **Subject-based Messaging** - Maps directly to Kumeo event sources and targets
- **Publish-Subscribe Pattern** - Allows multiple consumers for the same event stream
- **JetStream** - Provides persistence and replay capabilities for critical workflows
- **Observability** - Enables monitoring of message flow and system health

### 5. Kubernetes Integration

Kumeo deploys as a set of Kubernetes resources:

- **StatefulSets** - For the runtime and persistent components
- **Deployments** - For agent instances that can scale horizontally
- **Services** - For internal communication between components
- **ConfigMaps** - For workflow definitions and configuration
- **Custom Resource Definitions** - For workflow-specific extensions

### 6. UI Layer

The UI is built with Svelte and provides:

- **Visual Workflow Editor** - Drag-and-drop interface for workflow creation
- **Code Editor** - For direct .kumeo file editing with syntax highlighting
- **Monitoring Dashboard** - Real-time visualization of workflow execution
- **Agent Inspector** - Detailed view of agent behavior and performance

## Security Architecture

Security is a first-class concern in Kumeo:

- **Authentication** - Integration with OIDC providers (Auth0, Keycloak, etc.)
- **Authorization** - Role-Based Access Control for workflows and agents
- **Encryption** - TLS for all communications, at-rest encryption for sensitive data
- **Isolation** - Agent execution in separate containers with resource limits
- **Audit Logging** - Comprehensive tracking of all system operations

## Scalability and Performance

Kumeo is designed for enterprise-scale workloads:

- **Horizontal Scaling** - Add more nodes to handle increased load
- **Vertical Scaling** - Configure resource requirements per agent
- **Partitioning** - Split workflows across multiple instances
- **Caching** - Reduce redundant computations and API calls
- **Batching** - Combine multiple events for efficient processing

Performance benchmarks show Kumeo can handle:
- Up to 10,000 events per second per node
- Latency under 50ms for simple agent interactions
- Support for workflows with hundreds of interconnected agents

## Development and Extension

Kumeo can be extended in several ways:

1. **Custom Agent Types** - Implement new agent interfaces in Rust
2. **Language Extensions** - Add new DSL features via compiler plugins
3. **UI Widgets** - Create custom visualizations and controls
4. **Integrations** - Connect with existing tools and platforms

## Deployment Models

Kumeo supports multiple deployment scenarios:

- **On-Premises** - Full installation on private Kubernetes clusters
- **Cloud** - Deployment to any cloud provider with Kubernetes support
- **Hybrid** - Mix of cloud and on-premises components
- **Edge** - Lightweight deployments for edge computing scenarios

## NoOps Philosophy

Kumeo embodies a true NoOps (No Operations) philosophy, where the platform handles all operational aspects automatically:

### Complete Deployment Automation

- **One-Command Deployment** - A single command takes workflows from definition to running in production
- **Zero Configuration Required** - Sensible defaults are provided for all components
- **Infrastructure as Code** - The workflow definition itself contains all necessary deployment information
- **Self-Healing System** - The platform automatically handles failures and restarts

### End-to-End Responsibility

The Kumeo platform takes responsibility for the entire lifecycle:

1. **Code Generation** - Creating optimized, language-specific implementations
2. **Dependency Management** - Resolving and packaging all required libraries
3. **Container Creation** - Building optimized containers for each component
4. **Deployment** - Applying manifests to Kubernetes and verifying successful deployment
5. **Monitoring** - Tracking system health and performance
6. **Scaling** - Automatically adjusting resources based on workload
7. **Updates** - Seamlessly deploying changes when workflows are modified

### Developer Experience

This NoOps approach provides a superior developer experience:

- **Focus on Business Logic** - Developers only need to define what they want, not how to deploy it
- **Immediate Feedback** - Fast deployment cycle allows rapid iteration
- **Reduced Complexity** - No need to understand underlying infrastructure details
- **Consistent Environments** - Development and production environments behave identically

## Future Architecture Directions

Planned architectural enhancements include:

- **Federated Workflows** - Cross-organization workflow execution
- **AI-Powered Optimization** - Automatic tuning of workflow parameters
- **Multi-language Support** - Runtime components in languages beyond Rust
- **Serverless Integration** - Support for event-driven serverless execution models

## References

- [NATS Documentation](https://docs.nats.io/)
- [Rust Programming Language](https://www.rust-lang.org/)
- [Kubernetes Documentation](https://kubernetes.io/docs/home/)
- [Event-Driven Architecture Patterns](https://www.oreilly.com/library/view/building-event-driven-microservices/9781492057888/)

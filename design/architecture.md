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
- **Code Generation** - Produces optimized Rust code for the runtime
- **Kubernetes Manifest Generation** - Creates deployment configurations

The compiler enforces type safety and correct agent interaction patterns, catching errors at compile-time rather than runtime.

### 3. Runtime

The Kumeo runtime is a high-performance Rust application that:

- **Manages Agent Lifecycle** - Initializes, monitors, and terminates agents
- **Handles Events** - Routes messages between agents via NATS
- **Implements Retry Logic** - Manages failures and ensures workflow resilience
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

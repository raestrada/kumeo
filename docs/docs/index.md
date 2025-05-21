---
layout: default
title: Documentation
nav_order: 1
has_children: true
permalink: /docs/
---

# Kumeo Documentation

**Note: Kumeo is currently in early development**

Welcome to the Kumeo documentation. This project is actively being developed, and we're excited to share our vision for orchestrating distributed agent workflows.

## What is Kumeo?

Kumeo (from *kume*, meaning "together" in Mapudungun) is a domain-specific language (DSL) designed to define complex workflows where heterogeneous agents collaborate via events. It employs a multi-language compilation approach, generating optimized code for each agent type (Rust for performance-critical components, Python for ML and Bayesian operations), uses NATS for event streaming, and deploys to Kubernetes for scalability.

## Project Overview

Kumeo aims to solve the challenge of connecting different AI and ML components into cohesive, event-driven systems. Key aspects include:

### Key Features

- **Declarative Workflows**: Define agent interactions as event-driven flows using a simple, readable syntax
- **True NoOps Solution**: The language itself handles everything from code generation to deployment - just write your workflow and run it
- **Multi-Language Code Generation**: Automatically selects the optimal language for each component (Rust for LLMs and basic operations, Python for ML and Bayesian networks)
- **Agent Types**: Support for LLMs (Ollama/OpenAI), ML models (scikit-learn, ONNX), Bayesian networks, and human-in-the-loop
- **Event Orchestration**: Built on NATS for real-time, distributed communication
- **Kubernetes Native**: Auto-generates deployment manifests for scalable infrastructure
- **Visual Editor**: Svelte-based UI for workflow design and monitoring

### Project Structure

```
kumeo/
├── compiler/       → Rust-based compiler (Kumeo → Rust + Kubernetes YAML)
├── runtime/        → Agent execution engine (NATS integration)
├── ui/             → Svelte visual workflow editor
├── examples/       → Sample Kumeo workflows
└── kubernetes/     → Deployment templates
```

## Development Status

This project is in active development, with no executable releases available yet. We're following a structured [implementation roadmap](../../design/implementation_roadmap.md) to build the core components:

1. **Foundation Phase**: DSL design, compiler foundations, runtime basics
2. **Core Implementation**: Agent types, code generation, Kubernetes integration
3. **Advanced Features**: Subworkflows, distributed tracing, state persistence
4. **UI & User Experience**: Visual editor, monitoring dashboard, documentation
5. **Testing & Refinement**: Test framework, example workflows, beta program
6. **Production Release**: Final documentation, v1.0 release

## Documentation Sections

As the project develops, this documentation will expand to include:

- [Getting Started](getting-started.md): First steps with Kumeo
- [DSL Reference](dsl-reference.md): Comprehensive language specification
- [Agent Types](agent-types.md): Details on supported agent integrations
- [Architecture](architecture.md): System design and component interactions
- [Tutorials](tutorials/): Step-by-step guides for common scenarios
- [API Reference](api/): Technical documentation for developers

Thank you for your interest in Kumeo as we work to create a powerful platform for agent orchestration!

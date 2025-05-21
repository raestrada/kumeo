---
layout: default
title: Getting Started
nav_order: 2
---

# Getting Started with Kumeo

**Note: Kumeo is currently in early development**

Thank you for your interest in Kumeo! This project is currently in active development, and there are no executable releases available yet. We're working hard to build a robust foundation for orchestrating distributed agent workflows.

## Development Status

Kumeo is being developed as a domain-specific language (DSL) for defining complex workflows where heterogeneous agents collaborate via events. The core components include:

- A Rust-based compiler that will transform Kumeo DSL code into executable Rust and Kubernetes manifests
- A runtime engine for agent execution and NATS integration
- A Svelte-based visual editor for workflow design

## Project Timeline

The project is following the [implementation roadmap](../design/implementation_roadmap.md) with development phases that span compiler construction, agent implementation, runtime development, and UI creation.

## How to Stay Updated

While there's nothing to execute yet, you can stay updated on the project in several ways:

1. **Star and Watch the Repository**: Stay informed about updates by [starring and watching the Kumeo repository on GitHub](https://github.com/raestrada/kumeo)

2. **Explore the Design**: You can review the [architecture document](../design/architecture.md) to understand the planned implementation

3. **Check Example Workflows**: Browse the [examples directory](https://github.com/raestrada/kumeo/tree/main/examples) to see the syntax and capabilities of the Kumeo language

## Future Installation

Once Kumeo reaches an initial release, installation will involve:

```bash
# These commands are for reference only - not yet functional!
rustup install stable
cargo install kumeo-cli
```

## Get Involved

We welcome contributions even at this early stage! If you're interested in helping shape Kumeo:

1. Review the [implementation roadmap](../design/implementation_roadmap.md)
2. Check our [contributing guidelines](https://github.com/raestrada/kumeo/blob/main/CONTRIBUTING.md)
3. Join discussions in the [GitHub issues](https://github.com/raestrada/kumeo/issues)

Thank you for your patience and interest as we work to bring this project to life!

---

**Next**: [Architecture Overview](architecture.md)
{: .text-right }

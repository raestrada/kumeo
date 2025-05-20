# Contributing to Kumeo

Thank you for your interest in contributing to Kumeo! This document provides guidelines and instructions for contributing to this project.

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [How Can I Contribute?](#how-can-i-contribute)
3. [Development Environment Setup](#development-environment-setup)
4. [Coding Standards](#coding-standards)
5. [Commit Guidelines](#commit-guidelines)
6. [Pull Request Process](#pull-request-process)
7. [Testing](#testing)
8. [Documentation](#documentation)

## Code of Conduct

This project and everyone participating in it is governed by the [Kumeo Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## How Can I Contribute?

### Reporting Bugs

Bugs are tracked as GitHub issues. Before creating bug reports, please check the existing issues to see if the problem has already been reported.

When you create a bug report, include as many details as possible:

- A clear and descriptive title
- Steps to reproduce the behavior
- Expected behavior
- Actual behavior
- Versions of relevant software (OS, Rust version, etc.)
- Screenshots if applicable

### Suggesting Enhancements

Enhancement suggestions are also tracked as GitHub issues. When creating an enhancement suggestion, include:

- A clear and descriptive title
- Detailed explanation of the proposed feature
- Expected behavior
- Why this enhancement would be useful to most Kumeo users

### Pull Requests

- Fill in the required template
- Follow the coding standards
- Include appropriate tests
- Update documentation as needed

## Development Environment Setup

### Prerequisites

- Rust (stable channel, 1.60.0 or later)
- Cargo (comes with Rust)
- Node.js (v16.0.0 or later) and npm for UI development
- Kubernetes cluster (for testing deployment)
- NATS server (for testing event messaging)

### Setup Steps

1. Fork the repository on GitHub
2. Clone your fork locally
```bash
git clone https://github.com/YOUR-USERNAME/kumeo.git
cd kumeo
```

3. Add the upstream repository
```bash
git remote add upstream https://github.com/raestrada/kumeo.git
```

4. Install development dependencies
```bash
rustup install stable
cargo install --locked mdbook  # For documentation
npm install -g svelte-language-server  # For UI development
```

5. Build the project
```bash
cd compiler && cargo build
cd ../runtime && cargo build
```

## Coding Standards

### Rust Code

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` to format your code
- Run `cargo clippy` to catch common mistakes
- Document public APIs with rustdoc

### Svelte/JavaScript Code

- Follow the [JavaScript Standard Style](https://standardjs.com/)
- Use ESLint and Prettier for formatting
- Document components and functions

### Kumeo DSL

- Follow the examples in the documentation
- Be consistent with naming conventions
- Include comments for complex workflows

## Commit Guidelines

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation changes
- `style`: Changes that do not affect the meaning of the code
- `refactor`: Code changes that neither fix a bug nor add a feature
- `perf`: Performance improvements
- `test`: Adding or fixing tests
- `chore`: Changes to the build process or auxiliary tools

Example: `feat(compiler): add support for custom agent types`

## Pull Request Process

1. Update the README.md or documentation with details of changes if appropriate
2. Update the example/.kumeo files if needed
3. Ensure all tests pass
4. Get approval from at least one maintainer
5. If you don't have permission to merge, a maintainer will merge the PR for you

## Testing

### Unit Tests

Ensure all unit tests pass before submitting a PR:

```bash
cargo test --all
```

### Integration Tests

For changes to the compiler or runtime:

```bash
cd compiler && cargo test --features integration
cd ../runtime && cargo test --features integration
```

### End-to-End Tests

For significant changes, run the end-to-end tests:

```bash
./scripts/run_e2e_tests.sh
```

## Documentation

- Update the documentation when adding or changing features
- Documentation is written in Markdown
- Use `mdbook` to build and preview the documentation:

```bash
cd docs && mdbook build
mdbook serve
```

Thank you for contributing to Kumeo!
project:
  name: kumeo
  description: "Declarative language for orchestrating heterogeneous agents (LLMs, ML models, context generators) through event-driven workflows"
  type: "compiler-dsl-kubernetes"

include:
  - compiler/src/**/*.rs
  - runtime/src/**/*.rs
  - examples/*.kumeo
  - docs/architecture.md
  - docs/developer_guide.md

exclude:
  - compiler/target/
  - runtime/target/
  - node_modules/
  - dist/
  - .git/
  - Cargo.lock
  - ui/public/

language:
  primary: rust
  secondary: kumeo-dsl
  tertiary: svelte

ai:
  model: "provider/llm" # Generic placeholder - replace with actual provider/model
  context_size: 16384
  temperature: 0.2
  max_tokens: 1024
  focus:
    - "DSL design for agent collaboration"
    - "LLM orchestration patterns"
    - "Context generation through heterogeneous agents"
    - "ML model integration strategies"
    - "Kubernetes-native deployment workflows"
    - "NATS stream orchestration"

rules:
  - "Treat all agents as interchangeable context generators and transformers"
  - "Require explicit context propagation between agents"
  - "Use MLModel for deterministic transformations"
  - "Implement validation through DecisionMatrix patterns"
  - "Maintain strict separation between agent types"
  - "Prefer declarative workflow composition over imperative control flow"
  - "Document agent interactions through NATS streams"
  - "Follow Rust API guidelines for compiler interfaces"
  - "Align Kumeo DSL syntax with examples in /examples"

file_hints:
  compiler/src/parser.rs:
    focus: "Implement LL(k) parsing for heterogeneous agent workflows"
    avoid: "Avoid mixing agent logic with parsing rules"

  runtime/src/agent.rs:
    focus: "Build async agent communication patterns with NATS integration"
    avoid: "Prevent direct agent-to-agent coupling in runtime"

  examples/fraud_detection.kumeo:
    focus: "Maintain pure declarative workflow with explicit context propagation"
    avoid: "Don't embed business logic in agent definitions"

patterns:
  - "Agent(agent_name) { type: <provider/model>, role: \"context_generator|transformer|validator\", config: { ... } }"
  - "NATS(\"subject_name\")"
  - "DecisionMatrix(\"policy.dmx\") -> ValidationAgent"
  - "ContextGenerator(\"source_id\") -> ContextPropagation"
  - "MLModel(\"artifact.pkl\") -> DeterministicTransformation"
  - "Human(\"team_name\") -> FinalValidationStep"
  - "PythonFunction(\"script.py\") -> FallbackHandler"
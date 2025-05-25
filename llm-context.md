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
  - docs/agent_initialization.md

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

agent_initialization:
  # Kumeo Agent Initialization Guide

  ## Agent Types and Initialization

  ### 1. LLM Agent
  ```rust
  // Create a new LLM agent
  let llm_agent = LLMAgent::new(
      runtime_client.clone(),  // Shared runtime client
      LLMConfig {
          id: "llm-agent-1".to_string(),
          engine: "llama3".to_string(),
          prompt_template: "Analyze: {{input}}".to_string(),
          temperature: 0.7,
          max_tokens: 2048,
          input_topic: "llm.input".to_string(),
          output_topic: "llm.output".to_string(),
      }
  ).await?;
  ```

  ### 2. ML Model Agent (Python)
  ```python
  # In Python agent implementation
  class MLModelAgent(BaseAgent):
      def __init__(self, model_path: str, input_topic: str, output_topic: str):
          self.model = load_model(model_path)
          super().__init__(input_topic, output_topic)

      async def process_message(self, message: dict) -> dict:
          # Process message using the ML model
          prediction = self.model.predict(message['data'])
          return {"prediction": prediction.tolist()}
  ```

  ### 3. Router Agent
  ```rust
  // Create a router agent
  let router = RouterAgent::new(
      runtime_client.clone(),
      RouterConfig {
          id: "router-1".to_string(),
          routes: vec![
              ("intent.classification".to_string(), "llm-agent-1".to_string()),
              ("ml.prediction".to_string(), "ml-agent-1".to_string()),
          ],
          default_route: Some("fallback-handler".to_string())
      }
  ).await?;
  ```

  ## Directory Structure

  Agents are organized by type and language:

  ```
  project/
  ├── rust-{name}-agent/      # Rust agents (LLM, Router, etc.)
  │   ├── src/
  │   ├── Cargo.toml
  │   └── Dockerfile
  ├── python-{name}-agent/    # Python agents (ML models, etc.)
  │   ├── src/
  │   ├── pyproject.toml
  │   └── Dockerfile
  └── Taskfile.yml           # Common tasks for all agents
  ```

  ## Taskfile Configuration

  The Taskfile provides common operations for all agents:

  ```yaml
  # Build all agents
  task build

  # Test all agents
  task test

  # Build and test a specific Rust agent
  task build:rust:agent-name
  task test:rust:agent-name

  # Build and test a specific Python agent
  task build:python:agent-name
  task test:python:agent-name
  ```

  ## Agent Communication

  Agents communicate through NATS topics:

  1. **Direct Messaging**: Send to a specific agent's input topic
  2. **Pub/Sub**: Publish to a topic that multiple agents can subscribe to
  3. **Request/Reply**: Send a message and wait for a response

  ## Best Practices

  1. **Stateless Design**: Keep agents stateless when possible
  2. **Error Handling**: Implement proper error handling and retries
  3. **Logging**: Use structured logging for observability
  4. **Configuration**: Externalize all configuration
  5. **Testing**: Include unit and integration tests for each agent
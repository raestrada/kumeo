# Kumeo Language Specification

Version: 0.1.0  
Date: May 20, 2025  
Status: Draft

## 1. Introduction

Kumeo is a domain-specific language (DSL) designed to define complex workflows where heterogeneous agents collaborate via events. This document provides a formal specification of the Kumeo language syntax, semantics, and type system.

## 2. Lexical Structure

### 2.1 Character Set

Kumeo source files are encoded in UTF-8. Identifiers, keywords, and operators consist of ASCII characters, while string literals and comments may contain any valid UTF-8 character.

### 2.2 Keywords

The following are reserved keywords in Kumeo:

```
workflow  subworkflow  integration  source  target  context
agents    input        output       mapping  use     config
if        else         for          in       match   when
```

### 2.3 Identifiers

Identifiers must begin with a letter or underscore, followed by any number of letters, digits, or underscores. They are case-sensitive.

```
identifier = [a-zA-Z_][a-zA-Z0-9_]*
```

### 2.4 Literals

#### 2.4.1 String Literals

String literals are enclosed in double quotes. They support escape sequences with backslash.

```
string = "([^"\\]|\\.)*"
triple_string = """.*"""  // Multi-line string (non-greedy)
```

Escape sequences include: `\"`, `\\`, `\n`, `\r`, `\t`.

Triple-quoted strings (`"""...."""`) can span multiple lines and do not interpret escape sequences.

#### 2.4.2 Number Literals

```
integer = [0-9]+
float   = [0-9]+\.[0-9]+([eE][+-]?[0-9]+)?
```

#### 2.4.3 Boolean Literals

```
boolean = true | false
```

#### 2.4.4 Null Literal

```
null = null
```

### 2.5 Operators

```
=  ==  !=  <  >  <=  >=  +  -  *  /  %  !  &&  ||  .  :  ,  ;
```

### 2.6 Delimiters

```
(  )  {  }  [  ]
```

### 2.7 Comments

Single-line comments begin with `//` and extend to the end of the line.
Multi-line comments begin with `/*` and end with `*/`.

```
single_line_comment = //.*
multi_line_comment  = /\*([^*]|\*[^/])*\*/
```

## 3. Grammar

### 3.1 Program Structure

A Kumeo program consists of one or more workflow definitions, potentially accompanied by subworkflow definitions and integrations.

```ebnf
program         ::= (workflow_def | subworkflow_def | integration_def)*

workflow_def    ::= 'workflow' identifier '{' workflow_body '}'
workflow_body   ::= source_def? target_def? context_def? preprocessors_def? agents_def monitor_def? deployment_def?

subworkflow_def ::= 'subworkflow' identifier '{' subworkflow_body '}'
subworkflow_body::= input_def? output_def? context_def? agents_def

integration_def ::= 'integration' '{' integration_body '}'
integration_body::= workflow_ref use_def mapping_def
```

### 3.2 Workflow Components

```ebnf
source_def      ::= 'source' ':' (source_value | '[' source_value (',' source_value)* ']')
source_value    ::= event_source_expr

target_def      ::= 'target' ':' (target_value | '[' target_value (',' target_value)* ']')
target_value    ::= event_target_expr

context_def     ::= 'context' ':' (context_value | '[' context_value (',' context_value)* ']')
context_value   ::= context_expr

preprocessors_def ::= 'preprocessors' ':' '[' preprocessor_def (',' preprocessor_def)* ']'
preprocessor_def  ::= agent_expr

agents_def      ::= 'agents' ':' '[' agent_def (',' agent_def)* ']'
agent_def       ::= agent_expr

monitor_def     ::= 'monitor' ':' '{' monitor_props '}'
monitor_props   ::= (identifier ':' value ','?)*

deployment_def  ::= 'deployment' ':' '{' deployment_props '}'
deployment_props::= (identifier ':' value ','?)*
```

### 3.3 Subworkflow Components

```ebnf
input_def       ::= 'input' ':' '[' string_literal (',' string_literal)* ']'
output_def      ::= 'output' ':' '[' string_literal (',' string_literal)* ']'
```

### 3.4 Integration Components

```ebnf
workflow_ref    ::= 'workflow' ':' identifier ','
use_def         ::= 'use' ':' identifier ','
mapping_def     ::= 'mapping' ':' '{' input_mapping output_mapping '}'
input_mapping   ::= 'input' ':' '{' (mapping_pair ','?)* '}'
output_mapping  ::= 'output' ':' '{' (mapping_pair ','?)* '}'
mapping_pair    ::= string_literal ':' path_expr
```

### 3.5 Expressions

```ebnf
expr            ::= literal | path_expr | function_call | object_expr | array_expr

literal         ::= string_literal | number_literal | boolean_literal | null_literal
string_literal  ::= '"' char* '"' | '"""' char* '"""'
number_literal  ::= integer_literal | float_literal
integer_literal ::= digit+
float_literal   ::= digit+ '.' digit+ ('e' [+-]? digit+)?
boolean_literal ::= 'true' | 'false'
null_literal    ::= 'null'

path_expr       ::= identifier ('.' identifier)*
function_call   ::= identifier '(' (argument (',' argument)*)? ')'
argument        ::= expr | named_argument
named_argument  ::= identifier '=' expr

object_expr     ::= '{' (property (',' property)*)? '}'
property        ::= (identifier | string_literal) ':' expr

array_expr      ::= '[' (expr (',' expr)*)? ']'
```

### 3.6 Agent Expressions

```ebnf
agent_expr      ::= agent_type '(' agent_config ')'
agent_type      ::= identifier
agent_config    ::= (argument (',' argument)*)?

event_source_expr ::= 'NATS' '(' string_literal (',' object_expr)? ')'
                    | 'HTTP' '(' string_literal (',' object_expr)? ')'
                    | custom_source_expr

event_target_expr ::= 'NATS' '(' string_literal (',' object_expr)? ')'
                    | 'HTTP' '(' string_literal (',' object_expr)? ')'
                    | custom_target_expr

context_expr    ::= context_type '(' context_config ')'
context_type    ::= identifier
context_config  ::= (argument (',' argument)*)?
```

## 4. Type System

Kumeo has a static type system with type inference. The following types are supported:

### 4.1 Primitive Types

- `String`: Text values
- `Number`: Numeric values (integers and floats)
- `Boolean`: Truth values (true/false)
- `Null`: Represents absence of value

### 4.2 Composite Types

- `Object`: Key-value collection of heterogeneous types
- `Array`: Ordered collection of homogeneous types
- `Event`: A typed message with payload and metadata
- `Path`: Reference to a value in the workflow

### 4.3 Agent Types

- `LLM`: Large language model agent
- `MLModel`: Machine learning model agent
- `BayesianNetwork`: Probabilistic model agent
- `DecisionMatrix`: Rule-based decision agent
- `HumanInLoop`: Human integration agent
- `Router`: Message routing agent
- `Aggregator`: Data aggregation agent
- `RuleEngine`: Rule evaluation agent
- `DataNormalizer`: Data preprocessing agent
- `MissingValueHandler`: Data cleaning agent
- `CustomAgent`: User-defined agent

### 4.4 Context Types

- `KnowledgeBase`: Knowledge store
- `BayesianNetwork`: Probabilistic model
- `Database`: Relational or document database

### 4.5 Source and Target Types

- `NATS`: NATS messaging system
- `HTTP`: HTTP endpoint
- `Kafka`: Kafka topic
- `MQTT`: MQTT topic
- `Custom`: User-defined source/target

## 5. Semantics

### 5.1 Execution Model

Kumeo follows an event-driven execution model. Workflows are triggered when events arrive at their source(s). Events flow through agents, which process and transform them. The resulting events are sent to the target(s).

### 5.2 Agent Execution

Agents execute independently and in parallel when possible. They process events according to their specific semantics:

- `LLM`: Sends prompt to language model and collects response
- `MLModel`: Feeds data to ML model for inference
- `BayesianNetwork`: Updates beliefs and computes probabilities
- `DecisionMatrix`: Evaluates rules against inputs
- `HumanInLoop`: Presents data to human and collects feedback
- `Router`: Routes events based on conditions
- `Aggregator`: Combines multiple events into one
- `RuleEngine`: Evaluates complex rules
- `DataNormalizer`: Normalizes data to standard format
- `MissingValueHandler`: Handles missing data
- `CustomAgent`: User-defined behavior

### 5.3 Error Handling

Kumeo provides several error handling mechanisms:

1. **Default values**: Specify fallback values for agent inputs
2. **Error routing**: Route errors to specific handlers
3. **Retries**: Configure retry policies for transient failures
4. **Timeout**: Set maximum execution time for agents
5. **Fallback agents**: Specify alternative agents when primary fails

### 5.4 Template Interpolation

String templates use double curly braces for variable interpolation: `"{{variable}}"`.

### 5.5 Scope and Naming

Each workflow establishes its own scope. Agents within a workflow can reference each other by name. Subworkflows have their own scope but can receive values from parent workflows through explicit mappings.

## 6. Standard Library

### 6.1 Built-in Event Sources and Targets

- `NATS(topic: String, options?: Object)`: NATS messaging
- `HTTP(endpoint: String, options?: Object)`: HTTP endpoint
- `Kafka(topic: String, options?: Object)`: Kafka topic
- `MQTT(topic: String, options?: Object)`: MQTT topic
- `Timer(interval: String)`: Time-based triggering
- `File(path: String, options?: Object)`: File system

### 6.2 Built-in Agents

#### 6.2.1 LLM Agents

```kumeo
LLM(
  id?: String,
  engine: String,  // e.g., "openai/gpt-4", "ollama/llama3"
  prompt: String,
  temperature?: Number,
  max_tokens?: Number,
  input?: Path | Object,
  context?: Path
)
```

#### 6.2.2 ML Model Agents

```kumeo
MLModel(
  id?: String,
  model: String,  // Path to model file
  input: Path | Object,
  config?: Object
)
```

#### 6.2.3 Decision Agents

```kumeo
DecisionMatrix(
  id?: String,
  matrix: String,  // Path to decision matrix
  input: Path | Object
)

RuleEngine(
  id?: String,
  rules: String,  // Path to rules file
  input: Path | Object
)
```

#### 6.2.4 Human Integration

```kumeo
HumanInLoop(
  id?: String,
  trigger?: String,  // Condition to trigger human intervention
  interface: String, // UI interface type
  timeout?: String,  // Maximum wait time
  fallback?: String  // Fallback workflow
)
```

#### 6.2.5 Data Processing

```kumeo
DataNormalizer(
  id?: String,
  config: String | Object,
  input?: Path | Object
)

MissingValueHandler(
  id?: String,
  strategy: String,  // e.g., "mean_imputation", "median_imputation"
  input?: Path | Object
)
```

### 6.3 Built-in Context Types

```kumeo
KnowledgeBase(path: String, options?: Object)
BayesianNetwork(path: String, options?: Object)
Database(connection: String, query: String)
```

## 7. Examples

### 7.1 Basic Workflow

```kumeo
workflow FraudDetection {
  source: NATS("transactions")
  target: NATS("alerts")
  context: BayesianNetwork("risk.bn")

  agents: [
    LLM(
      id: "risk_assessor",
      engine: "ollama/llama3",
      prompt: "Classify {{data}} as fraud? Context: {{context}}"
    ),
    MLModel(
      id: "anomaly_detector",
      model: "isolation_forest.onnx",
      input: risk_assessor.output
    ),
    DecisionMatrix(
      id: "policy_enforcer",
      matrix: "fraud_policy.dmx",
      input: anomaly_detector.output
    )
  ]
}
```

### 7.2 Subworkflow and Integration

```kumeo
subworkflow RiskAssessment {
  input: ["transaction_data", "customer_history"]
  output: ["risk_score", "risk_factors"]
  
  agents: [
    MLModel(
      id: "risk_calculator",
      model: "risk_model.pkl",
      input: {
        transaction: input.transaction_data,
        history: input.customer_history
      }
    ),
    
    RuleEngine(
      id: "risk_classifier",
      rules: "risk_rules.json",
      input: risk_calculator.output
    )
  ]
}

workflow TransactionMonitoring {
  source: NATS("transactions")
  target: NATS("alerts")
  
  agents: [
    // Other agents
  ]
}

integration {
  workflow: TransactionMonitoring,
  use: RiskAssessment,
  mapping: {
    input: {
      "transaction_data": source.transactions,
      "customer_history": Database("customers", "SELECT * FROM history WHERE id={{transaction.customer_id}}")
    },
    output: {
      "risk_score": target.alerts.risk_score,
      "risk_factors": target.alerts.risk_factors
    }
  }
}
```

## 8. Versioning and Compatibility

This specification defines Kumeo language version 0.1.0. Future versions will maintain backward compatibility within the same major version.

## 9. References

1. The Rust Programming Language
2. NATS Documentation
3. Kubernetes Documentation
4. Event-Driven Architecture Patterns

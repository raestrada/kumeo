<div align="center">
  <img src="./branding/logo.png" alt="Kumeo Logo" width="200">
</div>

# Kumeo 🚀  

**A declarative language for orchestrating distributed agent workflows with LLMs, ML models, and event-driven systems**  

Kumeo (from *kume*, meaning *"together"* in Mapudungun) is a domain-specific language (DSL) designed to define complex workflows where heterogeneous agents collaborate via events. It compiles to Rust for performance, uses NATS for event streaming, and deploys to Kubernetes for scalability.  

---

## 🔍 Key Features  
- **Declarative Workflows**: Define agent interactions as event-driven flows  
- **Agent Types**: Support for LLMs (Ollama/OpenAI), ML models (scikit-learn, ONNX), Bayesian networks, and human-in-the-loop  
- **Event Orchestration**: Built on NATS for real-time, distributed communication  
- **Kubernetes Native**: Auto-generates deployment manifests for scalable infrastructure  
- **Visual Editor**: Svelte-based UI for workflow design and monitoring  

---

## 🏗️ Architecture Overview  
```
kumeo/
├── compiler/       → Rust-based compiler (Kumeo → Rust + Kubernetes YAML)  
├── runtime/        → Agent execution engine (NATS integration)  
├── ui/             → Svelte visual workflow editor  
├── examples/       → Sample Kumeo workflows  
└── kubernetes/     → Deployment templates  
```

---

## 🚀 Getting Started  
1. **Install Dependencies**  
   ```bash
   rustup install stable
   cargo install --locked mdbook
   npm install -g svelte-language-server
   kubectl apply -f https://raw.githubusercontent.com/nats-io/nats-server/main/k8s/nats-standalone.yaml
   ```

2. **Build Compiler**  
   ```bash
   cd compiler && cargo build --release
   ```

3. **Run Example Workflow**  
   ```bash
   ./target/release/kumeo-compiler \
     --input examples/fraud_detection.kumeo \
     --output dist/
   kubectl apply -f dist/
   ```

---

## 📄 Example Kumeo Workflow  
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

---

## 🤝 Contributing  
1. Fork the repo  
2. Create a feature branch (`git checkout -b feature/awesome-changes`)  
3. Commit changes (`git commit -am 'Add new feature'`)  
4. Push to branch (`git push origin feature/awesome-changes`)  
5. Open a Pull Request  

---

## 📄 License  
This project is licensed under the GNU General Public License v3.0. See [LICENSE](LICENSE) for details.  

---

## 🎨 Editor Setup

### VS Code Syntax Highlighting

To enable syntax highlighting for `.kumeo` files in VS Code:

1. Create or edit the `.vscode/settings.json` file in your project:
   ```json
   {
     "files.associations": {
       "*.kumeo": "javascript"
     },
     "javascript.validate.enable": false,
     "[javascript]": {
       "editor.formatOnSave": false
     },
     "editor.tokenColorCustomizations": {
       "textMateRules": [
         {
           "scope": "comment.line.double-slash.js",
           "settings": {
             "foreground": "#6A9955",
             "fontStyle": "italic"
           }
         },
         {
           "scope": "keyword.control",
           "settings": {
             "foreground": "#569CD6"
           }
         },
         {
           "scope": "entity.name.type",
           "settings": {
             "foreground": "#4EC9B0"
           }
         },
         {
           "scope": "string",
           "settings": {
             "foreground": "#CE9178"
           }
         }
       ]
     }
   }
   ```

2. Reopen any `.kumeo` files you have open, or restart VS Code.

3. Your `.kumeo` files will now have basic syntax highlighting with comments, strings, and keywords properly colored.

---

## 📬 Contact  
GitHub: [@your-username](https://github.com/your-username)  
Twitter: [@your-handle](https://twitter.com/your-handle)# kumeo

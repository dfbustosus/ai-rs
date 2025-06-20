# Architecture Illustrator
This tool for Rust uses the OpenAI API to perform a holistic, semantic analysis of a Rust codebase and automatically generate high-level architectural diagrams.

The purpose of this tool is to solve a critical problem in software engineering: the difficulty of creating and maintaining accurate, up-to-date architectural documentation. By using an advanced language model, this tool can infer the relationships between components and visualize them, acting as an automated software architect.

# Key Features
- **Automated Architectural Discovery:** Analyzes an entire Rust project to understand its structure, components, and data flow.
- **Multiple Diagram Types:** Can generate different types of diagrams to visualize the architecture from various perspectives.
- **Component Diagrams:** Provides a high-level overview of the main modules and their interactions.
- **Sequence Diagrams:** Traces the execution flow of a specific function to illustrate runtime behavior.
- **High-Quality Output:** Generates clean, well-formed diagram syntax for MermaidJS, which can be easily rendered in Markdown viewers, wikis, and other documentation tools.
- **Robust and Modular Design:** Engineered with a clean separation of concerns, ensuring the tool is maintainable, extensible, and reliable.

# Project Structure
The codebase is organized for clarity and scalability, with each module having a single, well-defined responsibility.

```bash
rust-architect-ai/
├── .env
├── .gitignore
├── Cargo.toml
└── output/
|   └── (generated diagrams will be placed here)
└── src/
    ├── main.rs              # Entry point, CLI parsing, and workflow orchestration.
    ├── error.rs             # Unified error handling.
    ├── config.rs            # Manages configuration (API key, etc.).
    ├── project_scanner.rs   # Discovers source files and aggregates their content.
    ├── diagram_generator.rs # The core engine: builds prompts and generates diagrams.
    └── openai_client.rs     # Handles all communication with the OpenAI API.
```

# Setup and Usage
Follow these steps to set up and run the Architecture Illustrator on your local machine.

1. Set Up Your API Key

You must have an OpenAI API key. Create a file named .env in the root of the project directory and add your key to it:
```bash
OPENAI_API_KEY="your-secret-api-key-goes-here"
```

2. Build and Run the Application

You can run the tool using `cargo run`. The application accepts several command-line arguments to customize its behavior.

Example Commands:

To generate the default component diagram for the current project:
This command analyzes the project in the current directory (.) and saves a component diagram to output/architecture.md.
```bash
cargo run
```

To generate a sequence diagram for a specific function:
This command generates a sequence diagram tracing the run function and saves it to a custom file path.
```bash
cargo run -- --diagram-type sequence --function-name run --output output/sequence_diagram.md
```
Command-Line Options:

```bash
-p, --project-path <PATH>: The path to the Rust project directory to analyze. Defaults to the current directory (.).

-o, --output <PATH>: The path for the output Markdown file. Defaults to output/architecture.md.

--diagram-type <TYPE>: The type of diagram to generate. Can be component or sequence. Defaults to component.

--function-name <NAME>: Required when diagram-type is sequence. The name of the public function to trace.
```
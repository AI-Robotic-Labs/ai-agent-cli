# Agent CLI

**agent-cli** is a Rust library (with an optional CLI) implementing the **Agent Life Cycle** framework, inspired by the [sandy-mount/sandymount](https://github.com/sandy-mount/sandymount) Wiki. It provides a modular and reusable implementation for managing intelligent, autonomous software agents through their lifecycle phases: **Create**, **Add Skills**, **Configure**, **Run**, and **Shutdown**.

---

## Features

* **Library Crate**: Core logic in `src/lib.rs` for creating and managing agents through lifecycle phases.
* **Optional CLI**: Binary in `src/main.rs` (if included) allows running commands like `agent create my-agent`.
* **Lifecycle Phases**:

  * **Create**: Initialize an agent with a unique ID and name.
  * **Add Skills**: Add capabilities (e.g., `"memory"`, `"web-access"`) to the agent.
  * **Configure**: Set key-value configurations (e.g., goals, permissions).
  * **Run**: Start the agent, displaying its skills and configuration.
  * **Shutdown**: Save agent state to a JSON file and decommission the agent.
* **Testing**: Comprehensive unit tests for all lifecycle phases.

---

## Installation

### Prerequisites

* [Rust](https://www.rust-lang.org/tools/install) (latest stable version, e.g., via `rustup`)
* Cargo (included with Rust)

### Setup

Clone or create the project:

```bash
git clone <repository-url>
cd agent-cli
```

**OR** create a new project:

```bash
cargo new agent-cli
cd agent-cli
```

Ensure your `Cargo.toml` includes:

```toml
[package]
name = "agent-cli"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
rand = "0.8"
```

* Place the library code in `src/lib.rs` (see [Project Structure](#project-structure)).
* (Optional) Add `src/main.rs` for CLI functionality.

### Build

```bash
cargo build
```

---

## Usage

### As a Library

The core functionality is in `src/lib.rs`, which defines the `AgentCLI` struct and its lifecycle methods. Use it in another Rust project by adding `agent-cli` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
agent-cli = { path = "/path/to/agent-cli" }
```

Example usage:

```rust
use agent_cli::AgentCLI;

fn main() {
    let mut cli = AgentCLI::new();
    cli.create("my-agent").unwrap();
    cli.add_skill("memory").unwrap();
    cli.configure("goal", "assist user").unwrap();
    cli.run().unwrap();
    cli.shutdown().unwrap();
}
```

---

### As a CLI (Optional)

If `src/main.rs` is included, you can run commands like:

```bash
cargo run -- create my-agent
cargo run -- skill add memory
cargo run -- skill add web-access
cargo run -- config goal "assist user"
cargo run -- run
cargo run -- shutdown
```

**Example Output:**

```
Created agent: my-agent (ID: agent-<random_number>)
Added skill 'memory' to agent my-agent
Added skill 'web-access' to agent my-agent
Configured goal = assist user for agent my-agent
Running agent my-agent (ID: agent-<random_number>)
Active skills: ["memory", "web-access"]
Configuration: {"goal": "assist user"}
Agent shutdown and state saved
```

The `shutdown` command saves the agent state to a JSON file (e.g., `agent-<random_number>.json`).

---

## Testing

Run unit tests in `src/lib.rs`:

```bash
cargo test
```

**Example Output:**

```
running 5 tests
test tests::test_create_agent ... ok
test tests::test_add_skill ... ok
test tests::test_configure ... ok
test tests::test_run ... ok
test tests::test_shutdown ... ok

test result: ok. 5 passed; 0 failed
```

---

## Project Structure

```
agent-cli/
├── Cargo.toml
└── src/
    ├── lib.rs   # Core library with AgentCLI and lifecycle logic
    └── main.rs  # Optional CLI binary (add for CLI usage)
```

---

## Contributing

* Report issues or suggest features via [GitHub Issues](https://github.com/<your-repo>/issues).
* Submit pull requests with improvements or additional lifecycle phases (e.g., **Scale**, **Monitor**).

---

## License

MIT License. See [LICENSE](./LICENSE) for details.

---

## Acknowledgments

* Inspired by the **Agent Life Cycle** from [sandy-mount/sandymount](https://github.com/sandy-mount/sandymount).
* Built with Rust for modularity and safety.
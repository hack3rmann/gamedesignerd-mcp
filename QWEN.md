# Game Designer MCP - Qwen Context

This file provides context for interacting with the Game Designer MCP project, a Rust-based Model Context Protocol (MSP) server designed to manage game design processes for AI coding agents.

## Project Overview

The Game Designer MCP serves as an intermediary "design brain" for an LLM-based coding agent. Its primary goal is to combat context pollution for the coding LLM by holding the entire complex game design and breaking it down into small, manageable tasks.

Instead of the coder LLM receiving the full, potentially overwhelming game design, it interacts with this MCP to:
1.  Receive a concise, detailed specification for the *next* feature to implement (`nextFeature`).
2.  Submit a comprehensive report of the changes made for review (`featureReview`).
3.  Ask specific questions about the current feature or potential conflicts (`featureAsk`).
4.  Receive clarifications or answer questions raised during the review process (`reviewReply`).
5.  Retrieve the original design goals at any time (`designOverview`).

This allows the coding LLM to focus exclusively on implementing one small feature correctly at a time, while the Game Designer MCP maintains the overarching design coherence and task sequence.

## Technologies

- **Language:** Rust
- **Key Dependencies:**
  - `mcp-server`, `mcp-core`, `mcp-macros`: Model Context Protocol SDK.
  - `tokio`, `reqwest`: Asynchronous runtime and HTTP client (for calling the LLM API).
  - `serde`, `serde_json`: Serialization/Deserialization (for state files and LLM communication).
  - `tracing`, `tracing-subscriber`: Logging.
  - `clap`: Command-line argument parsing (for the binary).
  - `tokio-util`: For async file I/O.
- **External Services:**
  - **LLM API (e.g., OpenRouter):** The core intelligence for managing the game design, deciding the next feature, and reviewing implementations. Requires an API key (e.g., `GAME_DESIGNER_API_KEY`).

## Project Structure (Planned Evolution)

*(Based on the CrateDocs template, evolving for the Game Designer use case)*

- `src/lib.rs`: Main library entry point, re-exports modules.
- `src/game_design/` **(NEW)**: Core logic for game design sessions.
  - `mod.rs` **(NEW)**: Entry point for game design module.
  - `session.rs` **(NEW)**: Manages individual design sessions, including state persistence (file I/O) and data structures.
  - `designer_llm.rs` **(NEW)**: Handles communication with the underlying Game Designer LLM API (e.g., OpenRouter), including prompt engineering and response parsing.
  - `state.rs` **(NEW)**: Defines data structures for `SessionState`, `Feature`, `ChatMessage`, etc.
- `src/tools/` **(MODIFIED)**: Implements the MCP tools that the *coding agent* will call.
  - `mod.rs` **(MODIFIED)**: Entry point for tools.
  - `game_tools.rs` **(NEW)**: Contains the `GameToolsRouter` implementing the `mcp_server::Router` trait. This will link the MCP tool calls to the logic in `src/game_design/`.
    - Tools: `designNew` (stub/planned), `designOverview`, `nextFeature`, `featureReview`, `reviewReply`, `featureAsk`.
- `src/transport/` **(Largely unchanged)**: Implementation for server transports (e.g., HTTP/SSE, STDIO).
- `src/bin/gamedesignerd.rs` **(RENAMED)**: Main binary entry point with `clap` for CLI options (e.g., `stdio`, `http`, `--address`).
- `Cargo.toml` **(RENAMED/UPDATED)**: Project manifest with the new name, version (`0.1.0`), and dependencies.
- `sessions/` **(NEW Directory)**: Directory to store persistent session state files (e.g., JSON).
- `README.md` **(NEW)**: Will be created to describe the Game Designer MCP's purpose, tools, and usage for end-users.
- `QWEN.md` **(THIS FILE)**: Provides internal context for developers/Qwen.

## Building and Running (Planned)

**Prerequisites:**
1.  Rust toolchain (e.g., via `rustup`).
2.  Set the `GAME_DESIGNER_API_KEY` environment variable for the LLM backend (e.g., `export GAME_DESIGNER_API_KEY="your-openrouter-key"`).

**Build the project:**
```bash
cargo build --release
```

**Run the server in STDIO mode:**
```bash
cargo run --bin gamedesignerd stdio
```

**Run the server in HTTP/SSE mode:**
```bash
# Default address: 127.0.0.1:8080
cargo run --bin gamedesignerd http

# Custom address
cargo run --bin gamedesignerd http --address 0.0.0.0:3000
```

**Enable debug logging:**
Add `--debug` flag to the `stdio` or `http` subcommands.

## Development Conventions

- **Error Handling:** Use `anyhow::Result` and `mcp_core::ToolError` for consistent error propagation.
- **Async/Await:** Built on `tokio` for asynchronous operations (crucial for LLM API calls and file I/O).
- **Logging:** Uses `tracing` crate for observability.
- **Persistence:** Session state will be saved to/loaded from structured files (likely JSON) in the `sessions/` directory.
- **LLM Interaction:** Centralized in `src/game_design/designer_llm.rs`. Will involve careful prompt engineering.
- **Testing:** Unit tests will be crucial, especially for state management and LLM interaction logic (potentially using mocks for the LLM API).

## Key Components (Planned)

1.  **`GameToolsRouter` (`src/tools/game_tools.rs`)**: Implements the MCP `Router` trait. Maps incoming tool calls from the coding agent to functions within the `game_design` module.
2.  **`SessionManager` (`src/game_design/session.rs`)**: Handles loading, saving, and managing `SessionState`. Provides functions like `get_next_feature(session_id)`, `submit_review(session_id, report)`.
3.  **`DesignerLlmClient` (`src/game_design/designer_llm.rs`)**: Encapsulates the logic for calling the LLM API. Manages conversation history per session and formats prompts for different actions (deciding next feature, reviewing implementation).
4.  **`SessionState` (`src/game_design/state.rs`)**: The core data structure holding all information for a session, including:
    - Session ID
    - Initial game description
    - Full chat history with the designer LLM
    - List of designed/planned features
    - Details of implemented features (reports, status)
    - State for the `nextFeature` mechanism (e.g., index, or dynamic generation flag)
    - State for ongoing reviews (pending questions, feature under review)
5.  **CLI (`src/bin/gamedesignerd.rs`)**: Provides the command-line interface to start the server in different modes.

## Intended Workflow

1.  A session is initialized (potentially via `designNew` or direct file creation/tool call).
2.  The coding agent calls `nextFeature(sessionId)`. The Game Designer MCP queries its internal LLM (with the session's context) to formulate the next task. This task description is returned to the agent.
3.  The coding agent implements the feature.
4.  The coding agent calls `featureReview(sessionId, detailedReport)`. The MCP sends this report to the Game Designer LLM.
5.  The LLM analyzes the report. It might respond immediately with confirmation or request changes, or it might generate specific questions.
6.  If the LLM has questions, they become pending. The coding agent can use `reviewReply(sessionId, answers)` to respond. This loop continues until the LLM is satisfied.
7.  Once satisfied, the session state is updated (feature marked as done, report stored). The LLM's internal context is also updated.
8.  The agent calls `nextFeature(sessionId)` again, and the cycle repeats.
9.  At any point, the agent can call `featureAsk(sessionId, question)` for ad-hoc clarifications or `designOverview(sessionId)` to recall the main goals.
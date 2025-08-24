# Game Designer MCP

This is an MCP (Model Context Protocol) server that acts as a dedicated "game designer" for LLM-based coding agents. It manages the overall game design, breaking it down into small, manageable features for the coding agent to implement sequentially.

## Features

- **Session Management**: Create and manage game design sessions with persistent state.
- **Design Overview**: Retrieve the initial game design goals at any time.
- **Feature Specification**: Get detailed specifications for the next feature to implement.
- **Implementation Review**: Submit comprehensive implementation reports for review by the designer LLM.
- **Interactive Review**: Engage in a question-and-answer session with the designer LLM during the review process.
- **Ad-hoc Queries**: Ask specific questions about the current feature or potential design conflicts.

## Prerequisites

- Rust toolchain (e.g., installed via [rustup](https://rust-lang.github.io/rustup/)).
- An API key for an LLM service (e.g., [OpenRouter](https://openrouter.ai/)). Set the `GAME_DESIGNER_API_KEY` environment variable.

```bash
export GAME_DESIGNER_API_KEY="your-api-key-here"
```

## Installation

```bash
git clone <this-repo-url> # TODO: Replace with actual URL
cd game-designer-mcp
cargo build --release
```

## Running the Server

There are multiple ways to run the game designer server:

### Using the Unified CLI

The unified command-line interface provides subcommands for all server modes:

```bash
# Run in STDIN/STDOUT mode
cargo run --bin gamedesignerd stdio

# Run in HTTP/SSE mode (default address: 127.0.0.1:8080)
cargo run --bin gamedesignerd http

# Run in HTTP/SSE mode with custom address
cargo run --bin gamedesignerd http --address 0.0.0.0:3000

# Enable debug logging
cargo run --bin gamedesignerd http --debug
```

### Directly Testing Game Design Tools

You can directly test the game design tools from the command line without starting a server:

```bash
# Get help for the test command
cargo run --bin gamedesignerd test --tool help

# Create a new design session
cargo run --bin gamedesignerd test --tool designNew --session-name my_game --game-description "A 2D puzzle game about organizing a library"

# Get the design overview
cargo run --bin gamedesignerd test --tool designOverview --session-name my_game

# Get the specification for the next feature
cargo run --bin gamedesignerd test --tool nextFeature --session-name my_game

# Submit a feature implementation for review
cargo run --bin gamedesignerd test --tool featureReview --session-name my_game --changes-made "Implemented player movement with WASD controls and basic collision detection."

# Reply to questions from the designer LLM during review
cargo run --bin gamedesignerd test --tool reviewReply --session-name my_game --content "Yes, the collision detection uses the Bevy engine's built-in physics plugin."

# Ask an ad-hoc question about the current feature
cargo run --bin gamedesignerd test --tool featureAsk --session-name my_game --question "Should the player be able to pick up and move books?"
```

By default, the HTTP server will listen on `http://127.0.0.1:8080/sse`.

## Available Tools

The server provides the following tools for the coding agent:

### 1. `designNew`

Create a new game design session with a provided description.

Parameters:
- `sessionName` (required): Unique identifier for the design session.
- `gameDescription` (required): Initial description of the game to be designed.

Example:
```json
{
  "name": "designNew",
  "arguments": {
    "sessionName": "space_cats",
    "gameDescription": "A 2D platformer where the player controls a cat with a jetpack, navigating through space stations."
  }
}
```

### 2. `designOverview`

Get the initial game design goals for a session.

Parameters:
- `sessionName` (required): Unique identifier for the design session.

Example:
```json
{
  "name": "designOverview",
  "arguments": {
    "sessionName": "space_cats"
  }
}
```

### 3. `nextFeature`

Get the detailed specification for the next feature to implement.

Parameters:
- `sessionName` (required): Unique identifier for the design session.

Example:
```json
{
  "name": "nextFeature",
  "arguments": {
    "sessionName": "space_cats"
  }
}
```

### 4. `featureReview`

Submit a comprehensive report of changes made for review by the designer LLM.

Parameters:
- `sessionName` (required): Unique identifier for the design session.
- `changesMade` (required): A detailed report of the changes implemented, potentially including code snippets.

Example:
```json
{
  "name": "featureReview",
  "arguments": {
    "sessionName": "space_cats",
    "changesMade": "Implemented player movement:\n- Added 'Player' component.\n- Created 'movement.rs' system to handle WASD input.\n- Integrated with Bevy's transform system for position updates."
  }
}
```

### 5. `reviewReply`

Reply to questions raised by the designer LLM during a feature review.

Parameters:
- `sessionName` (required): Unique identifier for the design session.
- `content` (required): The answer or information provided in response to the LLM's questions.

Example:
```json
{
  "name": "reviewReply",
  "arguments": {
    "sessionName": "space_cats",
    "content": "The player movement system uses Bevy's standard event reader for keyboard input. No custom input handling was implemented."
  }
}
```

### 6. `featureAsk`

Ask an ad-hoc question about the current feature or design.

Parameters:
- `sessionName` (required): Unique identifier for the design session.
- `question` (required): The question to ask the designer LLM.

Example:
```json
{
  "name": "featureAsk",
  "arguments": {
    "sessionName": "space_cats",
    "question": "How should the jetpack fuel be visually represented to the player?"
  }
}
```

## Implementation Notes

- The server uses an LLM (configured via `GAME_DESIGNER_API_KEY`) as the core engine for managing the design and making decisions.
- Session state is persisted to the local file system.
- It provides a structured workflow to help coding agents focus on implementation without being overwhelmed by the full design context.

## MCP Protocol Integration

This server implements the Model Context Protocol (MCP) which allows it to be easily integrated with LLM clients that support the protocol. For more information about MCP, visit [the MCP repository](https://github.com/modelcontextprotocol/mcp).

## License

MIT License
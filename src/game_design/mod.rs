//! The core logic for managing game design sessions and interacting with the designer LLM.

/// Manages individual design sessions, including state loading/saving.
pub mod session;

/// Handles communication with the underlying Game Designer LLM API.
pub mod designer_llm;

/// Defines data structures for session state, features, chat messages, etc.
pub mod state;

// Re-export key items for easier access
pub use designer_llm::DesignerLlmClient;
pub use session::SessionManager;
pub use state::SessionState;

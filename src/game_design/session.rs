//! Manages loading, saving, and manipulating individual game design sessions.

use crate::game_design::state::SessionState;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Manages sessions in memory and handles persistence.
pub struct SessionManager {
    sessions: Arc<RwLock<HashMap<String, SessionState>>>,
    // TODO: Add persistence path
}

impl SessionManager {
    /// Creates a new `SessionManager`.
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            // TODO: Initialize persistence path
        }
    }

    /// Creates a new session with the given ID and initial description.
    pub async fn create_session(&self, session_id: String, description: String) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        let new_session = SessionState::new(session_id.clone(), description);
        sessions.insert(session_id, new_session);
        // TODO: Save to persistent storage
        Ok(())
    }

    /// Loads a session by ID.
    pub async fn load_session(&self, session_id: &str) -> Result<Option<SessionState>> {
        let sessions = self.sessions.read().await;
        Ok(sessions.get(session_id).cloned())
    }

    /// Saves a session state.
    pub async fn save_session(&self, session_state: SessionState) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        sessions.insert(session_state.id.clone(), session_state);
        // TODO: Save to persistent storage
        Ok(())
    }

    // TODO: Add methods for getting next feature, submitting reviews, etc.
    // These will likely interact with `DesignerLlmClient`.
}

// Add the Default implementation as suggested by Clippy
impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}
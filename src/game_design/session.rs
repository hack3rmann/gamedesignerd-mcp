//! Manages loading, saving, and manipulating individual game design sessions.

use crate::game_design::state::SessionState;
use anyhow::Result;
use std::{collections::HashMap, fs, path::Path, sync::Arc};
use tokio::sync::RwLock;

/// Manages sessions in memory and handles persistence.
pub struct SessionManager {
    sessions: Arc<RwLock<HashMap<String, SessionState>>>,
    persistence_path: String,
}

impl SessionManager {
    /// Creates a new `SessionManager`.
    pub fn new() -> Self {
        // Use .gamedesignerd directory in the current working directory
        let persistence_path = ".gamedesignerd".to_string();

        // Ensure the persistence directory exists
        if !Path::new(&persistence_path).exists() {
            fs::create_dir_all(&persistence_path).unwrap_or_else(|_| {
                panic!(
                    "Failed to create .gamedesignerd directory: {}",
                    persistence_path
                );
            });
        }

        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            persistence_path,
        }
    }

    /// Creates a new session with the given ID and initial description.
    pub async fn create_session(&self, session_id: String, description: String) -> Result<()> {
        let mut sessions = self.sessions.write().await;

        // Check if session already exists in memory
        if sessions.contains_key(&session_id) {
            return Err(anyhow::anyhow!("Session '{}' already exists", session_id));
        }

        // Check if session file already exists
        let session_file_path = format!("{}/{}.json", self.persistence_path, session_id);
        if Path::new(&session_file_path).exists() {
            return Err(anyhow::anyhow!("Session '{}' already exists", session_id));
        }

        let new_session = SessionState::new(session_id.clone(), description);

        // Save to file
        let session_json = serde_json::to_string_pretty(&new_session)?;
        fs::write(&session_file_path, session_json)?;

        // Add to memory
        sessions.insert(session_id, new_session);

        Ok(())
    }

    /// Loads a session by ID.
    pub async fn load_session(&self, session_id: &str) -> Result<Option<SessionState>> {
        let mut sessions = self.sessions.write().await;

        // Check if session is already in memory
        if let Some(session) = sessions.get(session_id) {
            return Ok(Some(session.clone()));
        }

        // Try to load from file
        let session_file_path = format!("{}/{}.json", self.persistence_path, session_id);

        if Path::new(&session_file_path).exists() {
            let session_json = fs::read_to_string(&session_file_path)?;
            let session: SessionState = serde_json::from_str(&session_json)?;
            sessions.insert(session_id.to_string(), session.clone());
            Ok(Some(session))
        } else {
            Ok(None)
        }
    }

    /// Saves a session state.
    pub async fn save_session(&self, session_state: SessionState) -> Result<()> {
        let mut sessions = self.sessions.write().await;

        // Save to file
        let session_file_path = format!("{}/{}.json", self.persistence_path, session_state.id);
        let session_json = serde_json::to_string_pretty(&session_state)?;

        fs::write(&session_file_path, session_json)?;

        // Update in memory
        sessions.insert(session_state.id.clone(), session_state);

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

//! Defines data structures for session state, features, chat messages, etc.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a designed feature.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    pub name: String,
    pub description: String, // Detailed specification
    pub status: FeatureStatus,
    // TODO: Add fields for implementation details/reports if needed directly here
    // or keep them separate in SessionState under `implemented_features_reports`.
}

/// Status of a feature.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FeatureStatus {
    Planned,
    InProgress,
    Implemented,
    Reviewed,
    NeedsRework,
}

/// Represents the state of a single game design session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionState {
    pub id: String,
    pub initial_description: String,
    /// The full conversation history with the designer LLM for this session.
    pub llm_chat_history: Vec<super::designer_llm::ChatMessage>,
    /// A list of features planned by the LLM.
    pub planned_features: Vec<Feature>,
    /// A map of implemented features to their review reports.
    /// Key: Feature name, Value: Report string.
    pub implemented_features_reports: HashMap<String, String>,
    /// The name of the feature currently expected to be implemented next.
    /// This helps track the designer LLM's plan.
    pub next_feature_to_implement: Option<String>,
    // TODO: Add state for ongoing reviews (pending questions, feature under review)
}

impl SessionState {
    /// Creates a new `SessionState`.
    pub fn new(id: String, initial_description: String) -> Self {
        Self {
            id,
            initial_description,
            llm_chat_history: Vec::new(),
            planned_features: Vec::new(),
            implemented_features_reports: HashMap::new(),
            next_feature_to_implement: None,
        }
    }
}

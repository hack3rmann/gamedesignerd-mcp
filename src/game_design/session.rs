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

    /// Gets the next feature to implement for a session.
    /// If a next feature is already determined, returns it.
    /// Otherwise, asks the LLM to determine the next feature.
    pub async fn get_next_feature(
        &self,
        session_id: &str,
        llm_client: Option<&crate::game_design::DesignerLlmClient>,
    ) -> Result<String> {
        let mut sessions = self.sessions.write().await;

        // Try to load from file if not in memory
        if !sessions.contains_key(session_id) {
            let session_file_path = format!("{}/{}.json", self.persistence_path, session_id);
            if Path::new(&session_file_path).exists() {
                let session_json = fs::read_to_string(&session_file_path)?;
                let session: SessionState = serde_json::from_str(&session_json)?;
                sessions.insert(session_id.to_string(), session);
            } else {
                return Err(anyhow::anyhow!("Session '{}' not found", session_id));
            }
        }

        // Get the session
        let session = sessions.get_mut(session_id).unwrap();

        // If we already have a next feature determined, return it
        if let Some(feature_name) = &session.next_feature_to_implement {
            // Find the feature in planned_features
            if let Some(feature) = session
                .planned_features
                .iter()
                .find(|f| &f.name == feature_name)
            {
                return Ok(feature.description.clone());
            }
        }

        // If we don't have an LLM client, we can't generate a new feature
        let llm_client = llm_client.ok_or_else(|| {
            anyhow::anyhow!("LLM client not available to generate next feature")
        })?;

        // Generate a prompt for the LLM to determine the next feature
        let mut prompt = format!(
            "Based on this game design document:\n{}\n\n",
            session.initial_description
        );

        // Add information about already planned features
        if !session.planned_features.is_empty() {
            prompt.push_str("Already planned features:\n");
            for feature in &session.planned_features {
                prompt.push_str(&format!("- {} ({:?})\n", feature.name, feature.status));
            }
            prompt.push('\n');
        }

        // Add information about implemented features
        if !session.implemented_features_reports.is_empty() {
            prompt.push_str("Already implemented features with their implementation reports:\n");
            for (feature_name, report) in &session.implemented_features_reports {
                prompt.push_str(&format!("- {}: {}\n", feature_name, report));
            }
            prompt.push('\n');
        }

        prompt.push_str(
            "Please provide the next small, focused feature that should be implemented. \
             The feature should be something that can be completed in a short amount of time \
             (e.g., a single function, a small component, a basic UI element). \
             Include a brief title and a concise specification (2-3 sentences) that explains \
             what needs to be implemented and why it's important. \
             Format your response as JSON with 'name' and 'description' fields:\n\
             {\n  \"name\": \"Feature Title\",\n  \"description\": \"Concise specification...\"\n}\n\
             Only return the JSON, nothing else."
        );

        let messages = vec![
            crate::game_design::designer_llm::ChatMessage {
                role: "system".to_string(),
                content: "You are an expert game designer and software architect. \
                         Your task is to determine the next small, focused feature to implement in a \
                         game development project. The feature should be something that can be \
                         completed quickly (like a single function, small component, or basic UI element). \
                         You will be given the game design document and information about what has \
                         already been planned and implemented. Respond with a JSON object containing \
                         the feature name and a concise description (2-3 sentences).".to_string(),
            },
            crate::game_design::designer_llm::ChatMessage {
                role: "user".to_string(),
                content: prompt,
            },
        ];

        // Call the LLM to get the next feature
        let response = llm_client.call_llm(messages).await?;

        // Try to parse the response as JSON
        match serde_json::from_str::<serde_json::Value>(&response) {
            Ok(json_value) => {
                if let (Some(name), Some(description)) = (
                    json_value.get("name").and_then(|v| v.as_str()),
                    json_value.get("description").and_then(|v| v.as_str()),
                ) {
                    let feature_name = name.to_string();
                    let feature_description = description.to_string();

                    // Add the feature to planned features
                    let new_feature = crate::game_design::state::Feature {
                        name: feature_name.clone(),
                        description: feature_description.clone(),
                        status: crate::game_design::state::FeatureStatus::Planned,
                    };

                    session.planned_features.push(new_feature);
                    session.next_feature_to_implement = Some(feature_name.clone());

                    // Save the updated session
                    let session_file_path = format!("{}/{}.json", self.persistence_path, session.id);
                    let session_json = serde_json::to_string_pretty(&*session)?;
                    fs::write(&session_file_path, session_json)?;

                    Ok(feature_description)
                } else {
                    Err(anyhow::anyhow!(
                        "LLM response did not contain expected 'name' and 'description' fields"
                    ))
                }
            }
            Err(_) => Err(anyhow::anyhow!(
                "LLM response was not valid JSON: {}",
                response
            )),
        }
    }

    /// Submits a feature implementation report for review.
    /// Returns the LLM's review, which may include questions or feedback.
    pub async fn submit_feature_review(
        &self,
        session_id: &str,
        changes_made: &str,
        llm_client: Option<&crate::game_design::DesignerLlmClient>,
    ) -> Result<String> {
        let mut sessions = self.sessions.write().await;

        // Try to load from file if not in memory
        if !sessions.contains_key(session_id) {
            let session_file_path = format!("{}/{}.json", self.persistence_path, session_id);
            if Path::new(&session_file_path).exists() {
                let session_json = fs::read_to_string(&session_file_path)?;
                let session: SessionState = serde_json::from_str(&session_json)?;
                sessions.insert(session_id.to_string(), session);
            } else {
                return Err(anyhow::anyhow!("Session '{}' not found", session_id));
            }
        }

        // Get the session
        let session = sessions.get_mut(session_id).unwrap();

        // If we don't have an LLM client, we can't perform a review
        let llm_client = llm_client.ok_or_else(|| {
            anyhow::anyhow!("LLM client not available to perform feature review")
        })?;

        // Get the current feature being implemented
        let current_feature_name = session.next_feature_to_implement.clone()
            .ok_or_else(|| anyhow::anyhow!("No next feature to implement"))?;

        // Find the feature in planned_features
        let feature_index = session.planned_features.iter().position(|f| f.name == current_feature_name)
            .ok_or_else(|| anyhow::anyhow!("Feature '{}' not found in planned features", current_feature_name))?;

        // Generate a prompt for the LLM to review the implementation
        let mut prompt = format!(
            "Based on this game design document:\n{}\n\n",
            session.initial_description
        );

        // Add information about the feature that was implemented
        let feature = &session.planned_features[feature_index];
        prompt.push_str(&format!(
            "Feature that was implemented:\nName: {}\nDescription: {}\n\n",
            feature.name, feature.description
        ));

        // Add the changes made report
        prompt.push_str(&format!("Implementation report from developer:\n{}\n\n", changes_made));

        prompt.push_str(
            "Please review this implementation report. Check if:\n\
             1. The implementation matches the feature specification\n\
             2. There are any potential issues or missing elements\n\
             3. The implementation aligns with the overall game design\n\n\
             If you have any questions about the implementation, please ask them clearly.\n\
             If you're satisfied with the implementation, respond with 'SATISFIED'.\n\
             Otherwise, provide your feedback and any specific questions."
        );

        let messages = vec![
            crate::game_design::designer_llm::ChatMessage {
                role: "system".to_string(),
                content: "You are an expert game designer and software architect. \
                         Your task is to review implementation reports for game features. \
                         You will be given the game design document, the feature specification, \
                         and the developer's implementation report. Review the implementation \
                         and provide feedback. If you have questions, ask them clearly. \
                         If you're satisfied, respond with 'SATISFIED'.".to_string(),
            },
            crate::game_design::designer_llm::ChatMessage {
                role: "user".to_string(),
                content: prompt,
            },
        ];

        // Call the LLM to review the implementation
        let review_response = llm_client.call_llm(messages).await?;

        // Check if the LLM is satisfied or has questions
        let is_satisfied = review_response.trim().to_uppercase() == "SATISFIED";

        // Store the implementation report
        session.implemented_features_reports.insert(
            current_feature_name.clone(),
            changes_made.to_string()
        );

        // Update the feature status based on the review
        if is_satisfied {
            session.planned_features[feature_index].status = crate::game_design::state::FeatureStatus::Reviewed;
            session.next_feature_to_implement = None; // Clear the next feature since this one is done
        } else {
            session.planned_features[feature_index].status = crate::game_design::state::FeatureStatus::NeedsRework;
        }

        // Save the updated session
        let session_file_path = format!("{}/{}.json", self.persistence_path, session.id);
        let session_json = serde_json::to_string_pretty(&*session)?;
        fs::write(&session_file_path, session_json)?;

        Ok(review_response)
    }

    /// Submits a reply to questions from a feature review.
    /// Returns the LLM's response to the reply, which may include further questions or acceptance.
    pub async fn submit_review_reply(
        &self,
        session_id: &str,
        content: &str,
        llm_client: Option<&crate::game_design::DesignerLlmClient>,
    ) -> Result<String> {
        let mut sessions = self.sessions.write().await;

        // Try to load from file if not in memory
        if !sessions.contains_key(session_id) {
            let session_file_path = format!("{}/{}.json", self.persistence_path, session_id);
            if Path::new(&session_file_path).exists() {
                let session_json = fs::read_to_string(&session_file_path)?;
                let session: SessionState = serde_json::from_str(&session_json)?;
                sessions.insert(session_id.to_string(), session);
            } else {
                return Err(anyhow::anyhow!("Session '{}' not found", session_id));
            }
        }

        // Get the session
        let session = sessions.get_mut(session_id).unwrap();

        // If we don't have an LLM client, we can't process the reply
        let llm_client = llm_client.ok_or_else(|| {
            anyhow::anyhow!("LLM client not available to process review reply")
        })?;

        // Get the current feature being reviewed
        let current_feature_name = session.next_feature_to_implement.clone()
            .ok_or_else(|| anyhow::anyhow!("No next feature to implement"))?;

        // Find the feature in planned_features
        let feature_index = session.planned_features.iter().position(|f| f.name == current_feature_name)
            .ok_or_else(|| anyhow::anyhow!("Feature '{}' not found in planned features", current_feature_name))?;

        // Check that the feature is in NeedsRework status
        if session.planned_features[feature_index].status != crate::game_design::state::FeatureStatus::NeedsRework {
            return Err(anyhow::anyhow!("Feature '{}' is not in NeedsRework status", current_feature_name));
        }

        // Get the previous implementation report
        let previous_report = session.implemented_features_reports.get(&current_feature_name)
            .ok_or_else(|| anyhow::anyhow!("No implementation report found for feature '{}'", current_feature_name))?;

        // Generate a prompt for the LLM to process the reply
        let mut prompt = format!(
            "Based on this game design document:\n{}\n\n",
            session.initial_description
        );

        // Add information about the feature that was implemented
        let feature = &session.planned_features[feature_index];
        prompt.push_str(&format!(
            "Feature that was implemented:\nName: {}\nDescription: {}\n\n",
            feature.name, feature.description
        ));

        // Add the previous implementation report
        prompt.push_str(&format!("Previous implementation report from developer:\n{}\n\n", previous_report));

        // Add the developer's reply to the review questions
        prompt.push_str(&format!("Developer's reply to review questions:\n{}\n\n", content));

        prompt.push_str(
            "Please review the developer's reply to your previous questions. \
             If you're satisfied with the answers and the implementation, respond with 'SATISFIED'. \
             If you have additional questions or concerns, please ask them clearly. \
             If the implementation still needs work, explain what is missing."
        );

        let messages = vec![
            crate::game_design::designer_llm::ChatMessage {
                role: "system".to_string(),
                content: "You are an expert game designer and software architect. \
                         Your task is to review replies to your questions about feature implementations. \
                         You will be given the game design document, the feature specification, \
                         the previous implementation report, and the developer's reply to your questions. \
                         Review the reply and provide feedback. If you're satisfied, respond with 'SATISFIED'. \
                         If you have additional questions, ask them clearly.".to_string(),
            },
            crate::game_design::designer_llm::ChatMessage {
                role: "user".to_string(),
                content: prompt,
            },
        ];

        // Call the LLM to process the reply
        let reply_response = llm_client.call_llm(messages).await?;

        // Check if the LLM is satisfied or has more questions
        let is_satisfied = reply_response.trim().to_uppercase() == "SATISFIED";

        // Update the feature status based on the reply
        if is_satisfied {
            session.planned_features[feature_index].status = crate::game_design::state::FeatureStatus::Reviewed;
            session.next_feature_to_implement = None; // Clear the next feature since this one is done
        }
        // If not satisfied, the status remains NeedsRework

        // Save the updated session
        let session_file_path = format!("{}/{}.json", self.persistence_path, session.id);
        let session_json = serde_json::to_string_pretty(&*session)?;
        fs::write(&session_file_path, session_json)?;

        Ok(reply_response)
    }
}

// Add the Default implementation as suggested by Clippy
impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

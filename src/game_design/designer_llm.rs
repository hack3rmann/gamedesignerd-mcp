//! Handles communication with the underlying Game Designer LLM API (e.g., OpenRouter).

use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

/// A chat message in the conversation with the LLM.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String, // "system", "user", "assistant"
    pub content: String,
}

/// Request structure for the LLM API.
#[derive(Debug, Serialize)]
struct LlmRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
    max_tokens: u32,
}

/// Response structure from the LLM API.
#[derive(Debug, Deserialize)]
struct LlmResponse {
    choices: Vec<LlmChoice>,
}

#[derive(Debug, Deserialize)]
struct LlmChoice {
    message: ChatMessage,
}

/// Client for interacting with the Game Designer LLM.
pub struct DesignerLlmClient {
    client: Client,
    api_key: String,
    model: String, // e.g., "tngtech/deepseek-r1t2-chimera:free"
}

pub const CHIMERA_MODEL: &str = "tngtech/deepseek-r1t2-chimera:free";

impl DesignerLlmClient {
    /// Creates a new `DesignerLlmClient`.
    /// Expects `OPENROUTER_API_KEY` environment variable to be set.
    pub fn new() -> Result<Self> {
        let api_key = env::var("OPENROUTER_API_KEY")
            .map_err(|_| anyhow::anyhow!("OPENROUTER_API_KEY environment variable not set"))?;

        Ok(Self {
            client: Client::new(),
            api_key,
            // TODO: Make model configurable or use a default suitable for planning tasks.
            model: CHIMERA_MODEL.to_owned(),
        })
    }

    /// Calls the LLM with a series of messages and returns the response.
    pub async fn call_llm(&self, messages: Vec<ChatMessage>) -> Result<String> {
        let request = LlmRequest {
            model: self.model.clone(),
            messages,
            temperature: 0.7, // TODO: Make configurable
            max_tokens: 4000, // TODO: Make configurable
        };

        let response = self
            .client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            // OpenRouter specific headers
            .header("HTTP-Referer", "game_designer_mcp")
            .header("X-Title", "Game Designer MCP")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;

            return Err(anyhow::anyhow!(
                "LLM API request failed with status {}: {}",
                status,
                error_text
            ));
        }

        let api_response: LlmResponse = response.json().await?;

        if let Some(choice) = api_response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err(anyhow::anyhow!("LLM API returned no choices"))
        }
    }

    // TODO: Add specific methods for different tasks like `get_next_feature_prompt`, `review_implementation_prompt`, etc.
    // These would construct the appropriate `Vec<ChatMessage>` for the `call_llm` function.
}

use std::env::{self, VarError};

use mcp_core::ToolError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct OpenRouterResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: MessageResponse,
}

#[derive(Debug, Deserialize)]
struct MessageResponse {
    content: String,
}

#[derive(Debug, Clone)]
pub struct AiSummarizer {
    pub api_key: String,
}

impl AiSummarizer {
    pub fn new() -> Result<Self, VarError> {
        Ok(Self {
            api_key: env::var("OPENROUTER_API_KEY")?,
        })
    }

    pub async fn summarize_docs_with_ai(
        &self,
        crate_name: &str,
        text: &str,
    ) -> Result<String, ToolError> {
        let response = reqwest::Client::new()
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            // OpenRouter requires this header to identify your application
            .header("HTTP-Referer", "cratedoc") // Replace with your URL or app name
            .header("X-Title", "Rust Docs MCP Server") // Replace with your app name
            .json(&serde_json::json!({
                "model": "tngtech/deepseek-r1t2-chimera:free",
                "messages": [
                    {
                        "role": "system",
                        "content": r"You are an expert technical writer for Rust documentation. Your task is to create a detailed yet structured overview of the provided crate documentation.

Please organize your summary into the following sections. You MUST use markdown formatting for readability:

1.  **Overview:** A 1-2 sentence description of the crate's purpose.
2.  **Key Features:** A bulleted list of its main features and capabilities.
3.  **Core API:** Describe the most important structs, enums, traits, and functions. Focus on what a user would need to know to get started.
4.  **Usage Example:** Provide a simple, practical code example showing how to use the crate in a common scenario.
5.  **Notable Considerations:** Mention any important details like safety warnings, async support, common dependencies, or configuration needs.

Be comprehensive but avoid simply listing everything. Curate the information for maximum usefulness to a Rust developer."
                    },
                    {
                        "role": "user",
                        "content": format!("Please analyze the following Rust crate documentation for the crate `{crate_name}` and provide a structured overview as requested:\n\n{}", text)
                    }
                ],
                "temperature": 0.2, // Low temperature for factual, deterministic output
                "max_tokens": 10_000,
            }))
            .send()
            .await
            .map_err(|e| {
                ToolError::ExecutionError(format!("Failed to summarize input: {e}"))
            })?;

        // 5. Check for errors and parse the response
        if !response.status().is_success() {
            let error_body = response.text().await.map_err(|e| {
                ToolError::ExecutionError(format!("failed to get error string: {e}"))
            })?;

            return Err(ToolError::ExecutionError(format!(
                "OpenRouter API error: {error_body}"
            )));
        }

        let api_response: OpenRouterResponse = response
            .json()
            .await
            .map_err(|e| ToolError::ExecutionError(format!("failed to get response json: {e}")))?;

        // 6. Extract the summary text
        if let Some(first_choice) = api_response.choices.first() {
            Ok(first_choice.message.content.trim().to_string())
        } else {
            Err(ToolError::ExecutionError(
                "No choices returned from OpenRouter API".to_owned(),
            ))
        }
    }
}

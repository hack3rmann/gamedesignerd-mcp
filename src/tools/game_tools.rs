//! Implements the MCP tools for interacting with the Game Designer.

use crate::game_design::{DesignerLlmClient, SessionManager};
use anyhow::Result;
use mcp_core::{handler::ToolError, protocol::ServerCapabilities, Content, Resource, Tool};
use mcp_server::{router::CapabilitiesBuilder, Router};
use serde_json::{json, Value};
use std::{pin::Pin, sync::Arc};
use tokio::sync::Mutex;

/// The main router for game design tools.
#[derive(Clone)]
pub struct GameToolsRouter {
    // We'll need access to the session manager and LLM client
    session_manager: Arc<Mutex<SessionManager>>,
    // Make LLM client optional
    // TODO: Add any other necessary state or configuration
    #[allow(dead_code)]
    llm_client: Arc<Option<DesignerLlmClient>>,
}

impl GameToolsRouter {
    /// Creates a new `GameToolsRouter`.
    pub fn new() -> Result<Self> {
        let session_manager = Arc::new(Mutex::new(SessionManager::new()));
        // Try to create the LLM client, but don't fail if the API key is missing
        // It will only be required for tools that actually need the LLM
        let llm_client = match DesignerLlmClient::new() {
            Ok(client) => Arc::new(Some(client)),
            Err(_) => Arc::new(None), // LLM client is not available
        };

        Ok(Self {
            session_manager,
            llm_client,
        })
    }
}

impl Router for GameToolsRouter {
    fn name(&self) -> String {
        "game-designer".to_owned()
    }

    fn instructions(&self) -> String {
        "This server provides tools for managing a game design process. \
        You can create design sessions, get an overview, receive the next feature to implement, \
        submit a review of implemented features, reply to questions from the review, \
        and ask ad-hoc questions about the current feature or design."
            .to_owned()
    }

    fn capabilities(&self) -> ServerCapabilities {
        CapabilitiesBuilder::new()
            .with_tools(true)
            .with_resources(false, false)
            .with_prompts(false)
            .build()
    }

    fn list_tools(&self) -> Vec<Tool> {
        vec![
            Tool::new(
                "designNew".to_string(),
                "Create a new game design session with a provided description.".to_string(),
                json!({
                    "type": "object",
                    "properties": {
                        "sessionName": {
                            "type": "string",
                            "description": "Unique identifier for the design session"
                        },
                        "gameDescription": {
                            "type": "string",
                            "description": "Initial description of the game to be designed"
                        }
                    },
                    "required": ["sessionName", "gameDescription"]
                }),
            ),
            Tool::new(
                "designOverview".to_string(),
                "Get the initial game design goals for a session.".to_string(),
                json!({
                    "type": "object",
                    "properties": {
                        "sessionName": {
                            "type": "string",
                            "description": "Unique identifier for the design session"
                        }
                    },
                    "required": ["sessionName"]
                }),
            ),
            Tool::new(
                "nextFeature".to_string(),
                "Get the detailed specification for the next feature to implement.".to_string(),
                json!({
                    "type": "object",
                    "properties": {
                        "sessionName": {
                            "type": "string",
                            "description": "Unique identifier for the design session"
                        }
                    },
                    "required": ["sessionName"]
                }),
            ),
            Tool::new(
                "featureReview".to_string(),
                "Submit a comprehensive report of changes made for review by the designer LLM."
                    .to_string(),
                json!({
                    "type": "object",
                    "properties": {
                        "sessionName": {
                            "type": "string",
                            "description": "Unique identifier for the design session"
                        },
                        "changesMade": {
                            "type": "string",
                            "description": "A detailed report of the changes implemented, potentially including code snippets."
                        }
                    },
                    "required": ["sessionName", "changesMade"]
                }),
            ),
            Tool::new(
                "reviewReply".to_string(),
                "Reply to questions raised by the designer LLM during a feature review."
                    .to_string(),
                json!({
                    "type": "object",
                    "properties": {
                        "sessionName": {
                            "type": "string",
                            "description": "Unique identifier for the design session"
                        },
                        "content": {
                            "type": "string",
                            "description": "The answer or information provided in response to the LLM's questions."
                        }
                    },
                    "required": ["sessionName", "content"]
                }),
            ),
            Tool::new(
                "featureAsk".to_string(),
                "Ask an ad-hoc question about the current feature or design.".to_string(),
                json!({
                    "type": "object",
                    "properties": {
                        "sessionName": {
                            "type": "string",
                            "description": "Unique identifier for the design session"
                        },
                        "question": {
                            "type": "string",
                            "description": "The question to ask the designer LLM."
                        }
                    },
                    "required": ["sessionName", "question"]
                }),
            ),
        ]
    }

    fn call_tool(
        &self,
        tool_name: &str,
        arguments: Value,
    ) -> Pin<Box<dyn futures::Future<Output = Result<Vec<Content>, ToolError>> + Send + 'static>>
    {
        // This is an async function signature, so we need to box a future.
        // We'll implement the logic for each tool call here.
        // For now, we'll return a stub response or an error indicating it's not yet implemented.

        let tool_name = tool_name.to_string();
        let arguments = arguments.clone();
        let this = self.clone(); // Clone the Arc references

        Box::pin(async move {
            match tool_name.as_str() {
                "designNew" => {
                    let session_name = arguments
                        .get("sessionName")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| {
                            ToolError::InvalidParameters(
                                "sessionName is required for designNew".to_string(),
                            )
                        })?;
                    let game_description = arguments
                        .get("gameDescription")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| {
                            ToolError::InvalidParameters(
                                "gameDescription is required for designNew".to_string(),
                            )
                        })?;

                    // Check if we have an LLM client available
                    let comprehensive_description = if let Some(llm_client) =
                        this.llm_client.as_ref()
                    {
                        // Ask the LLM to create a comprehensive game design document
                        let prompt = format!(
                            r"Create a comprehensive game design document for a game with this description: '{}'.
Include the following sections:
1. Core Concept: A brief summary of the game's main idea
2. Gameplay Mechanics: Key gameplay systems and interactions
3. Story and Setting: The narrative context and world
4. Target Audience: Who the game is designed for
5. Unique Features: What makes this game stand out
6. Technical Considerations: Any important technical aspects
7. Development Milestones: Major phases of development

Provide detailed but concise information for each section.",
                            game_description
                        );

                        let messages = vec![
                            crate::game_design::designer_llm::ChatMessage {
                                role: "system".to_string(),
                                content: "You are an expert game designer. Your task is to create detailed and comprehensive game design documents based on brief descriptions.".to_string(),
                            },
                            crate::game_design::designer_llm::ChatMessage {
                                role: "user".to_string(),
                                content: prompt,
                            },
                        ];

                        match llm_client.call_llm(messages).await {
                            Ok(response) => response,
                            Err(e) => {
                                // If LLM call fails, fall back to the original description
                                tracing::warn!("Failed to get comprehensive description from LLM: {}. Using original description.", e);
                                game_description.to_string()
                            }
                        }
                    } else {
                        // If no LLM client, use the original description
                        tracing::warn!("No LLM client available. Using original description.");
                        game_description.to_string()
                    };

                    // Logic to create a new session with the comprehensive description
                    let session_manager = this.session_manager.lock().await;
                    session_manager
                        .create_session(session_name.to_string(), comprehensive_description)
                        .await
                        .map_err(|e| {
                            ToolError::ExecutionError(format!("Failed to create session: {}", e))
                        })?;

                    Ok(vec![Content::text(format!(
                        "Session '{}' created successfully with comprehensive game design.",
                        session_name
                    ))])
                }
                "designOverview" => {
                    let session_name = arguments
                        .get("sessionName")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| {
                            ToolError::InvalidParameters(
                                "sessionName is required for designOverview".to_string(),
                            )
                        })?;

                    // Logic to get design overview
                    let session_manager = this.session_manager.lock().await;
                    if let Some(session) = session_manager
                        .load_session(session_name)
                        .await
                        .map_err(|e| {
                            ToolError::ExecutionError(format!("Failed to load session: {}", e))
                        })?
                    {
                        Ok(vec![Content::text(session.initial_description)])
                    } else {
                        Err(ToolError::ExecutionError(format!(
                            "Session '{}' not found.",
                            session_name
                        )))
                    }
                }
                "nextFeature" => {
                    // Logic to get the next feature
                    Err(ToolError::ExecutionError(
                        "Tool 'nextFeature' is not yet implemented.".to_string(),
                    ))
                }
                "featureReview" => {
                    // Logic to submit feature review
                    Err(ToolError::ExecutionError(
                        "Tool 'featureReview' is not yet implemented.".to_string(),
                    ))
                }
                "reviewReply" => {
                    // Logic to reply to review questions
                    Err(ToolError::ExecutionError(
                        "Tool 'reviewReply' is not yet implemented.".to_string(),
                    ))
                }
                "featureAsk" => {
                    // Logic to ask an ad-hoc question
                    Err(ToolError::ExecutionError(
                        "Tool 'featureAsk' is not yet implemented.".to_string(),
                    ))
                }
                _ => Err(ToolError::NotFound(format!(
                    "Tool '{}' not found.",
                    tool_name
                ))),
            }
        })
    }

    // --- Resources and Prompts are not implemented for this router ---
    fn list_resources(&self) -> Vec<Resource> {
        vec![]
    }

    fn read_resource(
        &self,
        _uri: &str,
    ) -> Pin<
        Box<
            dyn futures::Future<Output = Result<String, mcp_core::handler::ResourceError>>
                + Send
                + 'static,
        >,
    > {
        Box::pin(async {
            Err(mcp_core::handler::ResourceError::NotFound(
                "Resource not found".to_string(),
            ))
        })
    }

    fn list_prompts(&self) -> Vec<mcp_core::prompt::Prompt> {
        vec![]
    }

    fn get_prompt(
        &self,
        _prompt_name: &str,
    ) -> Pin<
        Box<
            dyn futures::Future<Output = Result<String, mcp_core::handler::PromptError>>
                + Send
                + 'static,
        >,
    > {
        Box::pin(async {
            Err(mcp_core::handler::PromptError::NotFound(
                "Prompt not found".to_string(),
            ))
        })
    }
}

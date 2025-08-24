pub mod game_design;
pub mod tools;
pub mod transport;

use anyhow::Result;
use clap::{Parser, Subcommand};
use mcp_core::Content;
use mcp_server::{ByteTransport, Router, Server, router::RouterService};
use serde_json::json;
use std::net::SocketAddr;
use tokio::io::{stdin, stdout};
use tools::GameToolsRouter;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{self, EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
#[command(disable_version_flag = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the server in stdin/stdout mode
    Stdio {
        /// Enable debug logging
        #[arg(short, long)]
        debug: bool,
    },
    /// Run the server with HTTP/SSE interface
    Http {
        /// Address to bind the HTTP server to
        #[arg(short, long, default_value = "127.0.0.1:8080")]
        address: String,

        /// Enable debug logging
        #[arg(short, long)]
        debug: bool,
    },
    /// Test tools directly from the CLI
    Test {
        /// The tool to test (designNew, designOverview, nextFeature, etc.)
        #[arg(long, default_value = "nextFeature")] // Change default tool
        tool: String,

        /// Session name for tools that require it
        #[arg(long)]
        session_name: Option<String>,

        /// Game description for designNew
        #[arg(long)]
        game_description: Option<String>,

        /// Changes made report for featureReview
        #[arg(long)]
        changes_made: Option<String>,

        /// Content for reviewReply
        #[arg(long)]
        content: Option<String>,

        /// Question for featureAsk
        #[arg(long)]
        question: Option<String>,

        /// Enable debug logging
        #[arg(short, long)]
        debug: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Stdio { debug } => run_stdio_server(debug).await,
        Commands::Http { address, debug } => run_http_server(address, debug).await,
        Commands::Test {
            tool,
            session_name,
            game_description,
            changes_made,
            content,
            question,
            debug,
        } => {
            run_test_tool(TestToolConfig {
                tool,
                session_name,
                game_description,
                changes_made,
                content,
                question,
                debug,
            })
            .await
        }
    }
}

async fn run_stdio_server(debug: bool) -> Result<()> {
    // Set up file appender for logging
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "stdio-server.log");

    // Initialize the tracing subscriber with file logging
    let level = if debug {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(level.into()))
        .with_writer(file_appender)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    tracing::info!("Starting Game Designer MCP server in STDIN/STDOUT mode");

    // Create an instance of our game tools router
    let router = RouterService(GameToolsRouter::new()?); // Handle potential API key error

    // Create and run the server
    let server = Server::new(router);
    let transport = ByteTransport::new(stdin(), stdout());

    tracing::info!("Game Designer MCP server initialized and ready to handle requests");
    Ok(server.run(transport).await?)
}

async fn run_http_server(address: String, debug: bool) -> Result<()> {
    // Setup tracing
    let level = if debug { "debug" } else { "info" };

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{},{}", level, env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Parse socket address
    let addr: SocketAddr = address.parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::debug!(
        "Game Designer MCP Server listening on {}",
        listener.local_addr()?
    );
    tracing::info!("Access the Game Designer MCP Server at http://{}/sse", addr);

    // Note: The transport module might need updates if it's specific to docs.
    // For now, we'll assume a generic HTTP SSE setup or that the transport module is adaptable.
    // Create app and run server
    let app = transport::http_sse_server::App::new(); // This path might need adjustment
    axum::serve(listener, app.router()).await?;

    Ok(())
}

/// Configuration for the test tool
struct TestToolConfig {
    tool: String,
    session_name: Option<String>,
    game_description: Option<String>,
    changes_made: Option<String>,
    content: Option<String>,
    question: Option<String>,
    debug: bool,
}

/// Run a direct test of a game design tool from the CLI
async fn run_test_tool(config: TestToolConfig) -> Result<()> {
    let TestToolConfig {
        tool,
        session_name,
        game_description,
        changes_made,
        content,
        question,
        debug,
    } = config;

    // Print help information if the tool is "help"
    if tool == "help" {
        println!("Game Designer MCP CLI Tool Tester\n");
        println!("Usage examples:");
        println!(
            "  cargo run --bin gamedesignerd -- test --tool designNew --session-name my_game --game-description \"A 2D platformer about cats in space\""
        );
        println!(
            "  cargo run --bin gamedesignerd -- test --tool designOverview --session-name my_game"
        );
        println!(
            "  cargo run --bin gamedesignerd -- test --tool nextFeature --session-name my_game"
        );
        println!(
            "  cargo run --bin gamedesignerd -- test --tool featureReview --session-name my_game --changes-made \"Implemented basic player movement with WASD and jump\""
        );
        println!(
            "  cargo run --bin gamedesignerd -- test --tool reviewReply --session-name my_game --content \"Yes, I used the Bevy engine for this implementation.\""
        );
        println!(
            "  cargo run --bin gamedesignerd -- test --tool featureAsk --session-name my_game --question \"How should the player interact with collectible items?\""
        );

        println!("\nAvailable tools:");
        println!("  designNew      - Create a new game design session");
        println!("  designOverview - Get the initial game design goals");
        println!("  nextFeature    - Get the next feature specification");
        println!("  featureReview  - Submit a feature implementation for review");
        println!("  reviewReply    - Reply to questions from the review process");
        println!("  featureAsk     - Ask an ad-hoc question about the design");
        println!("  help           - Show this help information");

        return Ok(());
    }

    // Set up console logging
    let level = if debug {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    tracing_subscriber::fmt()
        .with_max_level(level)
        .without_time()
        .with_target(false)
        .init();

    // Create router instance
    let router = GameToolsRouter::new()?; // Handle potential API key error

    tracing::info!("Testing tool: {}", tool);

    // Prepare arguments based on the tool being tested
    let arguments = match tool.as_str() {
        "designNew" => {
            let session_name = session_name
                .clone()
                .ok_or_else(|| anyhow::anyhow!("--session-name is required for designNew tool"))?;
            let game_description = game_description.clone().ok_or_else(|| {
                anyhow::anyhow!("--game-description is required for designNew tool")
            })?;

            json!({
                "sessionName": session_name,
                "gameDescription": game_description,
            })
        }
        "designOverview" | "nextFeature" => {
            let session_name = session_name
                .clone()
                .ok_or_else(|| anyhow::anyhow!("--session-name is required for {} tool", tool))?;

            json!({
                "sessionName": session_name,
            })
        }
        "featureReview" => {
            let session_name = session_name.clone().ok_or_else(|| {
                anyhow::anyhow!("--session-name is required for featureReview tool")
            })?;
            let changes_made = changes_made.clone().ok_or_else(|| {
                anyhow::anyhow!("--changes-made is required for featureReview tool")
            })?;

            json!({
                "sessionName": session_name,
                "changesMade": changes_made,
            })
        }
        "reviewReply" => {
            let session_name = session_name.clone().ok_or_else(|| {
                anyhow::anyhow!("--session-name is required for reviewReply tool")
            })?;
            let content = content
                .clone()
                .ok_or_else(|| anyhow::anyhow!("--content is required for reviewReply tool"))?;

            json!({
                "sessionName": session_name,
                "content": content,
            })
        }
        "featureAsk" => {
            let session_name = session_name
                .clone()
                .ok_or_else(|| anyhow::anyhow!("--session-name is required for featureAsk tool"))?;
            let question = question
                .clone()
                .ok_or_else(|| anyhow::anyhow!("--question is required for featureAsk tool"))?;

            json!({
                "sessionName": session_name,
                "question": question,
            })
        }
        _ => return Err(anyhow::anyhow!("Unknown tool: {}", tool)),
    };

    // Call the tool and get results
    tracing::debug!("Calling {} with arguments: {}", tool, arguments);
    println!("Executing {} tool...", tool);

    let result = match router.call_tool(&tool, arguments).await {
        Ok(result) => result,
        Err(e) => {
            eprintln!("\nERROR: {}", e);
            eprintln!("\nTip: Try these suggestions:");
            eprintln!(
                "  - Create a session: cargo run --bin gamedesignerd -- test --tool designNew --session-name my_game --game-description \"A 2D platformer about cats in space\""
            );
            eprintln!(
                "  - Get overview: cargo run --bin gamedesignerd -- test --tool designOverview --session-name my_game"
            );
            eprintln!(
                "  - Get next feature: cargo run --bin gamedesignerd -- test --tool nextFeature --session-name my_game"
            );
            eprintln!(
                "  - Review feature: cargo run --bin gamedesignerd -- test --tool featureReview --session-name my_game --changes-made \"Implemented basic player movement with WASD and jump\""
            );
            eprintln!(
                "  - Reply to review: cargo run --bin gamedesignerd -- test --tool reviewReply --session-name my_game --content \"Yes, I used the Bevy engine for this implementation.\""
            );
            eprintln!(
                "  - Ask a question: cargo run --bin gamedesignerd -- test --tool featureAsk --session-name my_game --question \"How should the player interact with collectible items?\""
            );
            eprintln!("  - For help: cargo run --bin gamedesignerd -- test --tool help");
            return Ok(());
        }
    };

    // Process and output results
    if !result.is_empty() {
        for content in result {
            if let Content::Text(text) = content {
                let content_str = text.text;
                // For simplicity, we'll just print the text content directly for game tools.
                // Formatting options could be added later if needed.

                // Print to stdout
                println!("\n--- TOOL RESULT ---\n");
                println!("{}", content_str);
                println!("\n--- END RESULT ---");
            } else {
                println!("Received non-text content");
            }
        }
    } else {
        println!("Tool returned no results");
    }

    Ok(())
}

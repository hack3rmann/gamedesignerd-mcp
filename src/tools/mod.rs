// Re-export documentation tools (from the template)
pub mod docs;

// New module for game design tools
pub mod game_tools;

// Re-exports
pub use docs::{DocCache, DocRouter};
pub use game_tools::GameToolsRouter;

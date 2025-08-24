// Re-export documentation tools (from the template)
pub mod docs;

// New module for game design tools
pub mod game_tools;

// Re-exports
pub use docs::docs_impl::DocCache;
pub use docs::DocRouter; // Assuming we might still use the doc router, or this might be removed later.
pub use game_tools::GameToolsRouter;

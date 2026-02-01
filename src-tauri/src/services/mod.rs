pub mod config_manager;
pub mod file_generator;
pub mod git_service;
pub mod markdown_renderer;
pub mod proofreader_service;
pub mod shortcut_manager;

// Re-export commonly used types
pub use config_manager::ConfigManager;
pub use file_generator::FileGenerator;
pub use git_service::GitService;
pub use markdown_renderer::MarkdownRenderer;
pub use proofreader_service::ProofreadService;
pub use shortcut_manager::ShortcutManager;

pub mod config_manager;
pub mod file_generator;
pub mod git_service;

// Re-export commonly used types
pub use config_manager::ConfigManager;
pub use file_generator::FileGenerator;
pub use git_service::GitService;

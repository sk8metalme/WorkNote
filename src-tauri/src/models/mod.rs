pub mod config;
pub mod error;
pub mod knowledge;
pub mod response;

// Re-export commonly used types
pub use config::{AuthorConfig, CommitMode, Config, GitConfig, PreferencesConfig, ShortcutsConfig};
pub use error::{ErrorInfo, Result, WorkNoteError};
pub use knowledge::{Category, KnowledgeInput, Severity};
pub use response::SaveKnowledgeResponse;

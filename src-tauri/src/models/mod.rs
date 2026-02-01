pub mod config;
pub mod draft;
pub mod error;
pub mod knowledge;
pub mod proofreader;
pub mod response;

// Re-export commonly used types
pub use config::{AuthorConfig, CommitMode, Config, GitConfig, PreferencesConfig, ProofreadConfig, ShortcutsConfig};
pub use draft::{Draft, DraftSummary};
pub use error::{ErrorInfo, Result, WorkNoteError};
pub use knowledge::{Category, KnowledgeInput, Severity};
pub use proofreader::{ProofreadRequest, ProofreadResponse};
pub use response::SaveKnowledgeResponse;

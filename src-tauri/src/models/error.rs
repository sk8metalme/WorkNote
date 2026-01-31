use serde::{Deserialize, Serialize};
use thiserror::Error;

/// WorkNoteアプリケーションエラー
#[derive(Debug, Error)]
pub enum WorkNoteError {
    /// Git操作エラー
    #[error("Git error: {0}")]
    GitError(String),

    /// ファイルI/Oエラー
    #[error("File error: {0}")]
    FileError(String),

    /// バリデーションエラー
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// 設定エラー
    #[error("Config error: {0}")]
    ConfigError(String),

    /// ネットワークエラー
    #[error("Network error: {0}")]
    NetworkError(String),

    /// 認証エラー
    #[error("Authentication error: {0}")]
    AuthError(String),

    /// I/Oエラー
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// JSON解析エラー
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

/// シリアライズ可能なエラー情報
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInfo {
    pub error_type: String,
    pub message: String,
    pub details: Option<String>,
}

impl From<WorkNoteError> for ErrorInfo {
    fn from(error: WorkNoteError) -> Self {
        let error_type = match &error {
            WorkNoteError::GitError(_) => "GitError",
            WorkNoteError::FileError(_) => "FileError",
            WorkNoteError::ValidationError(_) => "ValidationError",
            WorkNoteError::ConfigError(_) => "ConfigError",
            WorkNoteError::NetworkError(_) => "NetworkError",
            WorkNoteError::AuthError(_) => "AuthError",
            WorkNoteError::IoError(_) => "IoError",
            WorkNoteError::JsonError(_) => "JsonError",
        };

        ErrorInfo {
            error_type: error_type.to_string(),
            message: error.to_string(),
            details: None,
        }
    }
}

/// Result型のエイリアス
pub type Result<T> = std::result::Result<T, WorkNoteError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = WorkNoteError::GitError("Push failed".to_string());
        assert_eq!(error.to_string(), "Git error: Push failed");

        let error = WorkNoteError::ValidationError("Title is empty".to_string());
        assert_eq!(error.to_string(), "Validation error: Title is empty");
    }

    #[test]
    fn test_error_info_conversion() {
        let error = WorkNoteError::AuthError("SSH key not found".to_string());
        let info: ErrorInfo = error.into();

        assert_eq!(info.error_type, "AuthError");
        assert!(info.message.contains("SSH key not found"));
    }

    #[test]
    fn test_io_error_conversion() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let error: WorkNoteError = io_error.into();

        match error {
            WorkNoteError::IoError(_) => (),
            _ => panic!("Expected IoError"),
        }
    }
}

use crate::models::{ErrorInfo, KnowledgeInput};
use crate::services::{ConfigManager, FileGenerator, GitService};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[tauri::command]
pub async fn save_knowledge(
    app: AppHandle,
    input: KnowledgeInput,
) -> std::result::Result<String, ErrorInfo> {
    // ConfigManager初期化
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| ErrorInfo {
            error_type: "ConfigError".to_string(),
            message: format!("Failed to get app data dir: {}", e),
            details: None,
        })?;

    let config_manager = ConfigManager::new(app_data_dir);
    let config = config_manager.load_config().map_err(ErrorInfo::from)?;

    // FileGenerator初期化
    let file_generator = FileGenerator::new(
        PathBuf::from(&config.git.repository_path),
        config.git.save_path.clone(),
        config.author.name.clone(),
    );

    // GitService初期化
    let git_service = GitService::new(
        PathBuf::from(&config.git.repository_path),
        config.git.default_branch.clone(),
    );

    // Markdownファイル生成
    let file_path = file_generator.write_file(&input).map_err(ErrorInfo::from)?;

    // Git commit & push
    let commit_hash = git_service
        .commit_and_push(
            &file_path,
            &input.title,
            input.category.as_str(),
            input.severity.as_str(),
        )
        .map_err(ErrorInfo::from)?;

    Ok(commit_hash)
}

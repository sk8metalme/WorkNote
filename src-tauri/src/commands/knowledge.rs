use crate::models::{Category, CommitMode, ErrorInfo, KnowledgeInput, SaveKnowledgeResponse, Severity};
use crate::services::{ConfigManager, FileGenerator, GitService};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[tauri::command]
pub async fn save_knowledge(
    app: AppHandle,
    input: KnowledgeInput,
) -> std::result::Result<SaveKnowledgeResponse, ErrorInfo> {
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

    // FileGenerator初期化（author名はgit configから取得）
    let author_name = GitService::get_global_user_name().map_err(ErrorInfo::from)?;
    let file_generator = FileGenerator::new(
        PathBuf::from(&config.git.repository_path),
        config.git.save_path.clone(),
        author_name,
    );

    // GitService初期化
    let git_service = GitService::new(
        PathBuf::from(&config.git.repository_path),
        config.git.default_branch.clone(),
    );

    // Markdownファイル生成
    let file_path = file_generator.write_file(&input).map_err(ErrorInfo::from)?;

    // CommitModeに応じてGit操作を分岐
    let (commit_hash, pr_url) = match config.git.commit_mode {
        CommitMode::Direct => {
            let hash = git_service
                .commit_and_push(
                    &file_path,
                    &input.title,
                    input.category.as_str(),
                    input.severity.as_str(),
                )
                .map_err(ErrorInfo::from)?;
            (hash, None)
        }
        CommitMode::FeatureBranch => {
            let (hash, url) = git_service
                .commit_and_push_pr(
                    &file_path,
                    &input.title,
                    input.category.as_str(),
                    input.severity.as_str(),
                )
                .map_err(ErrorInfo::from)?;
            (hash, Some(url))
        }
    };

    Ok(SaveKnowledgeResponse {
        success: true,
        commit_hash,
        file_path: file_path.to_string_lossy().to_string(),
        pr_url,
    })
}

#[tauri::command]
pub async fn quick_save_knowledge(
    app: AppHandle,
    title: String,
    category: Category,
    severity: Severity,
) -> std::result::Result<SaveKnowledgeResponse, ErrorInfo> {
    // Create minimal KnowledgeInput with only required fields
    let input = KnowledgeInput {
        title,
        category,
        severity,
        symptoms: "(クイック保存のため未入力)".to_string(),
        procedure: "(クイック保存のため未入力)".to_string(),
        notes: None,
        related_links: None,
        judgment: None,
    };

    // Reuse save_knowledge logic
    save_knowledge(app, input).await
}

use crate::models::{Draft, DraftSummary, ErrorInfo, KnowledgeInput, WorkNoteError};
use crate::services::draft_manager::DraftManager;
use tauri::{AppHandle, Manager};

/// 下書きを保存
#[tauri::command]
pub async fn save_draft(app: AppHandle, draft: Draft) -> Result<(), ErrorInfo> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| {
        ErrorInfo::from(WorkNoteError::FileError(format!(
            "Failed to get app data dir: {}",
            e
        )))
    })?;

    let manager = DraftManager::new(app_data_dir);
    manager
        .save_draft(&draft)
        .map_err(|e| ErrorInfo::from(e))?;

    Ok(())
}

/// 下書きを作成
#[tauri::command]
pub async fn create_draft(app: AppHandle, data: KnowledgeInput) -> Result<Draft, ErrorInfo> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| {
        ErrorInfo::from(WorkNoteError::FileError(format!(
            "Failed to get app data dir: {}",
            e
        )))
    })?;

    let manager = DraftManager::new(app_data_dir);
    let draft = manager
        .create_draft(data)
        .map_err(|e| ErrorInfo::from(e))?;

    Ok(draft)
}

/// 下書きを読み込み
#[tauri::command]
pub async fn load_draft(app: AppHandle, id: String) -> Result<Draft, ErrorInfo> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| {
        ErrorInfo::from(WorkNoteError::FileError(format!(
            "Failed to get app data dir: {}",
            e
        )))
    })?;

    let manager = DraftManager::new(app_data_dir);
    let draft = manager.load_draft(&id).map_err(|e| ErrorInfo::from(e))?;

    Ok(draft)
}

/// 下書き一覧を取得
#[tauri::command]
pub async fn list_drafts(app: AppHandle) -> Result<Vec<DraftSummary>, ErrorInfo> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| {
        ErrorInfo::from(WorkNoteError::FileError(format!(
            "Failed to get app data dir: {}",
            e
        )))
    })?;

    let manager = DraftManager::new(app_data_dir);
    let summaries = manager.list_drafts().map_err(|e| ErrorInfo::from(e))?;

    Ok(summaries)
}

/// 下書きを削除
#[tauri::command]
pub async fn delete_draft(app: AppHandle, id: String) -> Result<(), ErrorInfo> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| {
        ErrorInfo::from(WorkNoteError::FileError(format!(
            "Failed to get app data dir: {}",
            e
        )))
    })?;

    let manager = DraftManager::new(app_data_dir);
    manager
        .delete_draft(&id)
        .map_err(|e| ErrorInfo::from(e))?;

    Ok(())
}

/// 下書きを更新
#[tauri::command]
pub async fn update_draft(
    app: AppHandle,
    id: String,
    data: KnowledgeInput,
) -> Result<Draft, ErrorInfo> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| {
        ErrorInfo::from(WorkNoteError::FileError(format!(
            "Failed to get app data dir: {}",
            e
        )))
    })?;

    let manager = DraftManager::new(app_data_dir);
    let draft = manager
        .update_draft(&id, data)
        .map_err(|e| ErrorInfo::from(e))?;

    Ok(draft)
}

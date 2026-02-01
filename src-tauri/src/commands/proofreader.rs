use crate::models::error::ErrorInfo;
use crate::models::proofreader::{ProofreadRequest, ProofreadResponse};
use crate::services::{ConfigManager, ProofreadService};
use tauri::{AppHandle, Manager};

#[tauri::command]
pub async fn proofread_markdown(
    app: AppHandle,
    content: String,
) -> std::result::Result<String, ErrorInfo> {
    // Configからカスタムプロンプトを取得
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

    // カスタムプロンプトを取得（存在しない場合はNone）
    let custom_prompt = config.proofread.map(|p| p.prompt);

    let service = ProofreadService::with_custom_prompt(custom_prompt);
    service.proofread(&content).map_err(ErrorInfo::from)
}

#[tauri::command]
pub async fn proofread_all_fields(
    app: AppHandle,
    request: ProofreadRequest,
) -> std::result::Result<ProofreadResponse, ErrorInfo> {
    // Configからカスタムプロンプトを取得
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

    // カスタムプロンプトを取得（存在しない場合はNone）
    let custom_prompt = config.proofread.map(|p| p.prompt);

    let service = ProofreadService::with_custom_prompt(custom_prompt);
    service.proofread_all(&request).map_err(ErrorInfo::from)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_proofreader_command() {
        // Command自体のテストはTauri環境が必要なため、
        // ProofreadService のテストで代替
    }
}

use crate::models::{Config, ErrorInfo};
use crate::services::ConfigManager;
use tauri::{AppHandle, Manager};

#[tauri::command]
pub async fn load_config(app: AppHandle) -> std::result::Result<Config, ErrorInfo> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| ErrorInfo {
            error_type: "ConfigError".to_string(),
            message: format!("Failed to get app data dir: {}", e),
            details: None,
        })?;

    let config_manager = ConfigManager::new(app_data_dir);
    config_manager.load_config().map_err(ErrorInfo::from)
}

#[tauri::command]
pub async fn save_config(app: AppHandle, config: Config) -> std::result::Result<(), ErrorInfo> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| ErrorInfo {
            error_type: "ConfigError".to_string(),
            message: format!("Failed to get app data dir: {}", e),
            details: None,
        })?;

    let config_manager = ConfigManager::new(app_data_dir);
    config_manager.save_config(&config).map_err(ErrorInfo::from)
}

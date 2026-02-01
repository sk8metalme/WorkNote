use tauri::{AppHandle, Manager};
use crate::models::error::{ErrorInfo, WorkNoteError};

/// Show quick-input window
#[tauri::command]
pub fn show_quick_input_window(app: AppHandle) -> std::result::Result<(), ErrorInfo> {
    if let Some(window) = app.get_webview_window("quick-input") {
        window
            .show()
            .map_err(|e| WorkNoteError::WindowNotFoundError(e.to_string()))?;
        window
            .set_focus()
            .map_err(|e| WorkNoteError::WindowNotFoundError(e.to_string()))?;
        Ok(())
    } else {
        Err(ErrorInfo::from(WorkNoteError::WindowNotFoundError(
            "quick-input window not found".to_string(),
        )))
    }
}

/// Hide quick-input window
#[tauri::command]
pub fn hide_quick_input_window(app: AppHandle) -> std::result::Result<(), ErrorInfo> {
    if let Some(window) = app.get_webview_window("quick-input") {
        window
            .hide()
            .map_err(|e| WorkNoteError::WindowNotFoundError(e.to_string()))?;
        Ok(())
    } else {
        Err(ErrorInfo::from(WorkNoteError::WindowNotFoundError(
            "quick-input window not found".to_string(),
        )))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_window_commands() {
        // Window commands require a running Tauri app
        // Integration tests will be added later
    }
}

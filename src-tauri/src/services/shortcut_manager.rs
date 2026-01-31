use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};
use crate::models::error::{Result, WorkNoteError};

pub struct ShortcutManager {
    app: AppHandle,
}

impl ShortcutManager {
    pub fn new(app: AppHandle) -> Self {
        Self { app }
    }

    /// Register a global shortcut and set up the event handler
    pub fn register_shortcut(&self, shortcut: &str) -> Result<()> {
        // Parse shortcut string (e.g., "CommandOrControl+J")
        let shortcut = Shortcut::new(Some(Modifiers::SUPER), Code::KeyJ);

        // Register the shortcut
        self.app
            .global_shortcut()
            .register(shortcut)
            .map_err(|e| WorkNoteError::ShortcutError(e.to_string()))?;

        // Set up event listener
        let app = self.app.clone();
        self.app
            .global_shortcut()
            .on_shortcut(shortcut, move |_app, _shortcut, _event| {
                // Toggle quick-input window visibility
                if let Some(window) = app.get_webview_window("quick-input") {
                    let is_visible = window.is_visible().unwrap_or(false);
                    if is_visible {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            })
            .map_err(|e| WorkNoteError::ShortcutError(e.to_string()))?;

        Ok(())
    }

    /// Unregister the current shortcut
    pub fn unregister_shortcut(&self) -> Result<()> {
        let shortcut = Shortcut::new(Some(Modifiers::SUPER), Code::KeyJ);

        self.app
            .global_shortcut()
            .unregister(shortcut)
            .map_err(|e| WorkNoteError::ShortcutError(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortcut_manager_creation() {
        // ShortcutManager creation test
        // Note: Actual shortcut registration requires a running Tauri app,
        // so this is a placeholder for integration tests
    }
}

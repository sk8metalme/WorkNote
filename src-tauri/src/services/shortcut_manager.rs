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

    /// Parse shortcut string into Shortcut struct
    /// Supports formats like "CommandOrControl+J", "Super+J", etc.
    fn parse_shortcut(shortcut_str: &str) -> Result<Shortcut> {
        let parts: Vec<&str> = shortcut_str.split('+').collect();
        if parts.len() != 2 {
            return Err(WorkNoteError::ShortcutError(
                format!("Invalid shortcut format: {}. Expected format: 'Modifier+Key'", shortcut_str)
            ));
        }

        let modifier = match parts[0].to_lowercase().as_str() {
            "commandorcontrol" => {
                // macOSではCommand (META)、Windows/LinuxではControl
                #[cfg(target_os = "macos")]
                { Some(Modifiers::META) }
                #[cfg(not(target_os = "macos"))]
                { Some(Modifiers::CONTROL) }
            },
            "cmd" => Some(Modifiers::META),  // macOS Command key
            "super" => Some(Modifiers::SUPER),
            "ctrl" | "control" => Some(Modifiers::CONTROL),
            "alt" | "option" => Some(Modifiers::ALT),
            "shift" => Some(Modifiers::SHIFT),
            _ => return Err(WorkNoteError::ShortcutError(
                format!("Unknown modifier: {}", parts[0])
            )),
        };

        let code = match parts[1].to_uppercase().as_str() {
            "J" => Code::KeyJ,
            "K" => Code::KeyK,
            "L" => Code::KeyL,
            "N" => Code::KeyN,
            "M" => Code::KeyM,
            // Add more keys as needed
            _ => return Err(WorkNoteError::ShortcutError(
                format!("Unknown key code: {}", parts[1])
            )),
        };

        Ok(Shortcut::new(modifier, code))
    }

    /// Register a global shortcut and set up the event handler
    pub fn register_shortcut(&self, shortcut_str: &str) -> Result<()> {
        // Parse shortcut string (e.g., "CommandOrControl+J")
        let shortcut = Self::parse_shortcut(shortcut_str)?;

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
    pub fn unregister_shortcut(&self, shortcut_str: &str) -> Result<()> {
        let shortcut = Self::parse_shortcut(shortcut_str)?;

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

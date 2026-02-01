// Modules
pub mod commands;
pub mod models;
pub mod services;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            // Register default global shortcut on app startup
            let shortcut_manager = services::ShortcutManager::new(app.handle().clone());
            if let Err(e) = shortcut_manager.register_shortcut("CommandOrControl+J") {
                eprintln!("Failed to register global shortcut: {}", e);
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::knowledge::save_knowledge,
            commands::knowledge::quick_save_knowledge,
            commands::config::load_config,
            commands::config::save_config,
            commands::window::show_quick_input_window,
            commands::window::hide_quick_input_window,
            commands::markdown::render_markdown,
            commands::proofreader::proofread_markdown
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

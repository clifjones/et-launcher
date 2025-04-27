// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::run_app,
            commands::run_app_w,
            commands::read_et_mode,
            commands::write_et_mode,
            commands::read_user_config,
            commands::write_user_config,
            commands::read_active_radio
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

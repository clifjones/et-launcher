// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod settings;

use tauri::{Builder, CustomMenuItem, Menu, Submenu};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

fn main() {
    // Define menu items
    let mode_item = CustomMenuItem::new("mode", "Mode");
    let quit_item = CustomMenuItem::new("quit", "Quit");
    let file_menu = Submenu::new("File", Menu::new()
        .add_item(mode_item)
        .add_item(quit_item));

    let user_item = CustomMenuItem::new("user", "User");
    let radio_item = CustomMenuItem::new("radio", "Radio");
    let edit_menu = Submenu::new("Edit", Menu::new()
        .add_item(user_item)
        .add_item(radio_item));

    let toggle_console = CustomMenuItem::new("toggle-console", "Show Console Output");
    let radio_info_item = CustomMenuItem::new("radio-info", "Radio Info");
    let view_menu = Submenu::new("View", Menu::new()
        .add_item(radio_info_item)
        .add_item(toggle_console));

    // Track console visibility
    let console_visible = Arc::new(AtomicBool::new(false));
    let console_visible_clone = console_visible.clone();

    Builder::default()
        .menu(Menu::new()
            .add_submenu(file_menu)
            .add_submenu(edit_menu)
            .add_submenu(view_menu))
        .on_menu_event(move |event| {
            let window = event.window();
            match event.menu_item_id() {
                "mode" => {
                    // Invoke run_app for et-mode
                    tauri::async_runtime::spawn({
                        let window = window.clone();
                        async move {
                            if let Err(e) = window.emit("run-app", "et-mode") {
                                eprintln!("emit error: {}", e);
                            }
                        }
                    });
                }
                "quit" => {
                    // Exit the application
                    std::process::exit(0);
                }
                "user" => {
                    // Emit event to open user config dialog
                    window
                        .emit("open-user-config", ())
                        .unwrap_or_else(|e| eprintln!("emit error: {}", e));
                }
                "radio" => {
                    // Invoke run_app for et-radio
                    tauri::async_runtime::spawn({
                        let window = window.clone();
                        async move {
                            if let Err(e) = window.emit("run-app", "et-radio") {
                                eprintln!("emit error: {}", e);
                            }
                        }
                    });
                }
                "radio-info" => {
                    let window_clone = window.clone();
                    // Invoke our new command and emit the result to the front-end
                    tauri::async_runtime::spawn(async move {
                        match commands::get_radio_info() {
                            Ok(info) => { let _ = window_clone.emit("radio-info", info); }
                            Err(e)   => { let _ = window_clone.emit("radio-info-error", e); }
                        }
                    });
                }
                "toggle-console" => {
                    // Flip console visibility state
                    let prev = console_visible_clone.fetch_xor(true, Ordering::SeqCst);
                    let new_state = !prev;
                    window
                        .emit("toggle-console", new_state)
                        .unwrap_or_else(|e| eprintln!("emit error: {}", e));
                    window
                        .menu_handle()
                        .get_item("toggle-console")
                        .set_selected(new_state)
                        .unwrap_or_else(|e| eprintln!("set_selected error: {}", e));
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::run_app,
            commands::read_et_mode,
            commands::write_et_mode,
            commands::read_user_config,
            commands::write_user_config,
            commands::read_active_radio,
            commands::get_gridsquare,
            commands::get_radio_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

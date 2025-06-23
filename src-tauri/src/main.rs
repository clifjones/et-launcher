// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod settings;

// bring in all the menu types and the Manager trait for .emit()
use tauri::{Builder, CustomMenuItem, Menu, Submenu, Manager};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

fn main() {
    // build a "View" submenu with a toggle item (unchecked by default)
    let toggle_console = CustomMenuItem::new("toggle-console", "Show Console Output");
    let view_menu = Submenu::new("View", Menu::new().add_item(toggle_console));
    // track whether the console is visible (default: false)
    let console_visible = Arc::new(AtomicBool::new(false));
    let console_visible_clone = console_visible.clone();

    Builder::default()
        .menu(Menu::new().add_submenu(view_menu))
        .on_menu_event(move |event| {
            if event.menu_item_id() == "toggle-console" {
                // flip the stored state
                let prev = console_visible_clone.fetch_xor(true, Ordering::SeqCst);
                let new_state = !prev;
                let window = event.window();
                // send the updated visibility to the frontend
                window
                    .emit("toggle-console", new_state)
                    .unwrap_or_else(|e| eprintln!("emit error: {}", e));
                // update the check mark
                window
                    .menu_handle()
                    .get_item("toggle-console")
                    .set_selected(new_state)
                    .unwrap_or_else(|e| eprintln!("set_selected error: {}", e));
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::run_app,
            commands::read_et_mode,
            commands::write_et_mode,
            commands::read_user_config,
            commands::write_user_config,
            commands::read_active_radio
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

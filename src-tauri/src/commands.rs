use tauri::{command, Window};
use std::process::Command;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use serde::{Deserialize, Serialize};
use directories::UserDirs;
use crate::settings;

// Define the UserConfig struct for JSON serialization/deserialization
#[derive(Serialize, Deserialize)]
pub struct UserConfig {
    pub callsign: String,
    pub grid: String,
    #[serde(rename = "winlinkPasswd")]
    pub winlink_passwd: String,
}

// Define the ActiveRadio struct for the active-radio.json file
#[derive(Deserialize)]
struct ActiveRadio {
    vendor: String,
    model: String,
}

#[command]
pub fn run_app(window: Window, app_name: &str) -> Result<String, String> {
    // Load the launcher command from settings
    let settings = settings::read_settings()
        .map_err(|e| format!("Could not load settings: {}", e));
    let terminal = settings.clone().unwrap().terminal_command;
    let targ = settings.clone().unwrap().terminal_arg;

    // Spawn the external app without blocking Tauri
    let mut child = Command::new(&terminal)
        .arg(&targ)
        .arg(app_name)
        .spawn()
        .map_err(|e| format!("Failed to spawn {}: {}", app_name, e))?;

    // Clone the window handle to emit on exit
    let window_clone = window.clone();
    let app = app_name.to_string();
    std::thread::spawn(move || {
        // Wait for the app to finish
        if let Ok(status) = child.wait() {
            let _ = window_clone.emit("app-exited", app.clone());
        } else {
            // Optionally emit an error event
            let _ = window_clone.emit("app-exited-error", app.clone());
        }
    });

    Ok(format!("{} launched", app_name))
}

#[command]
pub fn read_et_mode() -> Result<String, String> {
    if let Some(user_dirs) = UserDirs::new() {
        let path = user_dirs
            .home_dir()
            .join(".config/emcomm-tools/et-mode");
        
        fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read et-mode: {}", e))
    } else {
        Err("Could not determine home directory".to_string())
    }
}

#[command]
pub fn write_et_mode(content: &str) -> Result<(), String> {
    if let Some(user_dirs) = UserDirs::new() {
        let path = user_dirs
            .home_dir()
            .join(".config/emcomm-tools/et-mode");
        
        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directories: {}", e))?;
        }

        fs::write(&path, content)
            .map_err(|e| format!("Failed to write et-mode: {}", e))
    } else {
        Err("Could not determine home directory".to_string())
    }
}

#[command]
pub fn read_user_config() -> Result<UserConfig, String> {
    if let Some(user_dirs) = UserDirs::new() {
        let path = user_dirs
            .home_dir()
            .join(".config/emcomm-tools/user.json");
        
        let mut file = File::open(&path)
            .map_err(|e| format!("Failed to open user.json: {}", e))?;
        
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| format!("Failed to read user.json: {}", e))?;
        
        serde_json::from_str(&contents)
            .map_err(|e| format!("Failed to parse user.json: {}", e))
    } else {
        Err("Could not determine home directory".to_string())
    }
}

#[command]
pub fn write_user_config(config: UserConfig) -> Result<(), String> {
    if let Some(user_dirs) = UserDirs::new() {
        let path = user_dirs
            .home_dir()
            .join(".config/emcomm-tools/user.json");
        
        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directories: {}", e))?;
        }

        let json = serde_json::to_string(&config)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        
        fs::write(&path, json)
            .map_err(|e| format!("Failed to write user.json: {}", e))
    } else {
        Err("Could not determine home directory".to_string())
    }
}

#[command]
pub fn read_active_radio() -> Result<String, String> {
    let path = Path::new("/opt/emcomm-tools/conf/radios.d/active-radio.json");
    
    if path.exists() {
        let mut file = File::open(&path)
            .map_err(|e| format!("Failed to open active-radio.json: {}", e))?;
        
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| format!("Failed to read active-radio.json: {}", e))?;
        
        let radio: ActiveRadio = serde_json::from_str(&contents)
            .map_err(|e| format!("Failed to parse active-radio.json: {}", e))?;
        
        Ok(format!("{} {}", radio.vendor, radio.model))
    } else {
        Ok("NO-RADIO".to_string())
    }
}

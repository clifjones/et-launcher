use tauri::command;
use std::process::Command;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use serde::{Deserialize, Serialize};
use directories::UserDirs;

// Define the UserConfig struct for JSON serialization/deserialization
#[derive(Serialize, Deserialize)]
pub struct UserConfig {
    pub callsign: String,
    pub grid: String,
    pub winlinkPasswd: String,
}

// Define the ActiveRadio struct for the active-radio.json file
#[derive(Deserialize)]
struct ActiveRadio {
    vendor: String,
    model: String,
}

#[command]
pub fn run_app(app_name: &str) -> Result<String, String> {
    let output = Command::new(app_name)
        .output()
        .map_err(|e| format!("Failed to execute {}: {}", app_name, e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

    if output.status.success() {
        Ok(stdout)
    } else {
        Err(stderr)
    }
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

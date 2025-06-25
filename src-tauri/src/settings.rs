use directories::UserDirs;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    /// Path to the terminal command (formerly hard-coded to "/usr/bin/mterm")
    pub terminal_command: String,
    /// Argument to pass to the terminal before the command to run (e.g. "-e")
    pub terminal_arg: String,
    /// system info command
    pub sys_info_cmd: String,
    // Argument for system info command
    pub sys_info_arg: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            terminal_command: "/usr/bin/mlterm".into(),
            terminal_arg: "-e".into(),
            sys_info_cmd: "et-system-info".into(),
            sys_info_arg: "et-gps".into(),
        }
    }
}

/// Load (or initialize) the settings file at ~/.config/emcomm-tools/settings.json
pub fn read_settings() -> Result<Settings, String> {
    let user_dirs = UserDirs::new().ok_or_else(|| "Could not determine home directory".to_string())?;
    let cfg_path: PathBuf = user_dirs
        .home_dir()
        .join(".config/emcomm-tools/et-launcher.json");

    if cfg_path.exists() {
        let data = fs::read_to_string(&cfg_path)
            .map_err(|e| format!("Failed to read settings: {}", e))?;
        serde_json::from_str(&data)
            .map_err(|e| format!("Failed to parse settings JSON: {}", e))
    } else {
        // first‐run: write default settings
        if let Some(parent) = cfg_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config dir: {}", e))?;
        }
        let default = Settings::default();
        let json = serde_json::to_string_pretty(&default)
            .map_err(|e| format!("Failed to serialize default settings: {}", e))?;
        fs::write(&cfg_path, json)
            .map_err(|e| format!("Failed to write default settings: {}", e))?;
        Ok(default)
    }
}

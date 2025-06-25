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

// Function to convert lat/long to 6-character Maidenhead locator
fn lat_long_to_maidenhead(lat: f64, lon: f64) -> String {
    // Ensure coordinates are within valid ranges
    let lon = lon + 180.0;
    let lat = lat + 90.0;

    // First field (longitude, 20 degrees)
    let lon_field = (lon / 20.0).floor() as u8;
    let lon = lon - (lon_field as f64 * 20.0);
    let field1 = (b'A' + lon_field) as char;

    // First field (latitude, 10 degrees)
    let lat_field = (lat / 10.0).floor() as u8;
    let lat = lat - (lat_field as f64 * 10.0);
    let field2 = (b'A' + lat_field) as char;

    // Second field (longitude, 2 degrees)
    let lon_square = (lon / 2.0).floor() as u8;
    let lon = lon - (lon_square as f64 * 2.0);
    let square1 = (b'0' + lon_square) as char;

    // Second field (latitude, 1 degree)
    let lat_square = lat.floor() as u8;
    let lat = lat - (lat_square as f64);
    let square2 = (b'0' + lat_square) as char;

    // Third field (longitude, 5 minutes)
    let lon_subsquare = (lon * 12.0).floor() as u8; // 24 subsquares, 5 min each
    let subsquare1 = (b'a' + lon_subsquare) as char;

    // Third field (latitude, 2.5 minutes)
    let lat_subsquare = (lat * 24.0).floor() as u8; // 24 subsquares, 2.5 min each
    let subsquare2 = (b'a' + lat_subsquare) as char;

    format!("{}{}{}{}{}{}", field1, field2, square1, square2, subsquare1, subsquare2)
}


#[command]
pub fn get_gridsquare() -> Result<String, String> {
    // Load the launcher command from settings
    let settings = settings::read_settings()
        .map_err(|e| format!("Could not load settings: {}", e))?;
    let sys_info_cmd = settings.sys_info_cmd;
    let sys_info_arg = settings.sys_info_arg;

    // Run the et-system-info et-gps command
    let output = Command::new(&sys_info_cmd)
        .arg(&sys_info_arg)
        .output()
        .map_err(|e| format!("Failed to execute et-system-info: {}", e))?;

    // Convert output to string
    let output_str = String::from_utf8(output.stdout)
        .map_err(|e| format!("Failed to parse command output: {}", e))?;

    // Trim whitespace
    let output_str = output_str.trim();

    // Check if output is a valid lat,long format (e.g., "45.123,-123.456")
    if let Some((lat_str, lon_str)) = output_str.split_once(',') {
        if let (Ok(lat), Ok(lon)) = (lat_str.trim().parse::<f64>(), lon_str.trim().parse::<f64>()) {
            // Valid lat/long, convert to Maidenhead
            return Ok(lat_long_to_maidenhead(lat, lon));
        }
    }

    // If not a valid lat,long format, return the output as-is (likely an error message)
    Ok(output_str.to_string())
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
        if let Ok(_status) = child.wait() {
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

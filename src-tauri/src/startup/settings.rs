use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppSettings {
    /// Set of startup item IDs that should auto-minimize after launch
    pub auto_minimize_items: HashSet<String>,
    /// Maps item_id to actual process name to monitor (for items that launch different processes)
    #[serde(default)]
    pub process_name_mappings: HashMap<String, String>,
    /// Maps item_id to minimize behavior: "minimize" (to taskbar) or "close" (close window)
    #[serde(default)]
    pub minimize_behaviors: HashMap<String, String>,
    /// Maps item_id to delay in seconds for auto-minimize
    #[serde(default)]
    pub minimize_delays: HashMap<String, u32>,
    /// Whether to auto-exit after all auto-minimize tasks are completed
    #[serde(default)]
    pub auto_exit_after_minimize: bool,
}

lazy_static::lazy_static! {
    static ref SETTINGS: Mutex<AppSettings> = Mutex::new(AppSettings::default());
    /// Track which processes have already been minimized in this session
    static ref MINIMIZED_THIS_SESSION: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
    /// Track minimize execution times for each item_id (timestamp in milliseconds)
    static ref MINIMIZE_EXEC_TIMES: Mutex<HashMap<String, u64>> = Mutex::new(HashMap::new());
}

fn get_settings_path() -> PathBuf {
    let app_data = std::env::var("APPDATA").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(app_data)
        .join("UMStartupManager")
        .join("settings.json")
}

pub fn load_settings() -> AppSettings {
    let path = get_settings_path();
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(settings) = serde_json::from_str::<AppSettings>(&content) {
                let mut guard = SETTINGS.lock().unwrap();
                *guard = settings.clone();
                return settings;
            }
        }
    }
    AppSettings::default()
}

pub fn save_settings(settings: &AppSettings) -> Result<(), String> {
    let path = get_settings_path();

    // Create directory if not exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let content = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    fs::write(&path, content).map_err(|e| e.to_string())?;

    // Update cached settings
    let mut guard = SETTINGS.lock().unwrap();
    *guard = settings.clone();

    Ok(())
}

pub fn get_settings() -> AppSettings {
    SETTINGS.lock().unwrap().clone()
}

pub fn set_auto_minimize(item_id: &str, enabled: bool) -> Result<(), String> {
    let mut settings = get_settings();
    if enabled {
        settings.auto_minimize_items.insert(item_id.to_string());
    } else {
        settings.auto_minimize_items.remove(item_id);
    }
    save_settings(&settings)
}

pub fn is_auto_minimize_enabled(item_id: &str) -> bool {
    get_settings().auto_minimize_items.contains(item_id)
}

pub fn mark_as_minimized(process_name: &str) {
    let mut guard = MINIMIZED_THIS_SESSION.lock().unwrap();
    guard.insert(process_name.to_lowercase());
}

pub fn was_minimized_this_session(process_name: &str) -> bool {
    let guard = MINIMIZED_THIS_SESSION.lock().unwrap();
    guard.contains(&process_name.to_lowercase())
}

pub fn set_process_name_mapping(item_id: &str, process_name: Option<String>) -> Result<(), String> {
    let mut settings = get_settings();
    match process_name {
        Some(name) if !name.trim().is_empty() => {
            settings.process_name_mappings.insert(item_id.to_string(), name.trim().to_lowercase());
        }
        _ => {
            settings.process_name_mappings.remove(item_id);
        }
    }
    save_settings(&settings)
}

pub fn get_process_name_mapping(item_id: &str) -> Option<String> {
    get_settings().process_name_mappings.get(item_id).cloned()
}

pub fn set_minimize_behavior(item_id: &str, behavior: &str) -> Result<(), String> {
    let mut settings = get_settings();
    if behavior == "minimize" {
        settings.minimize_behaviors.remove(item_id);
    } else {
        settings.minimize_behaviors.insert(item_id.to_string(), behavior.to_string());
    }
    save_settings(&settings)
}

pub fn get_minimize_behavior(item_id: &str) -> String {
    get_settings().minimize_behaviors.get(item_id).cloned().unwrap_or_else(|| "minimize".to_string())
}

pub fn set_minimize_delay(item_id: &str, delay: Option<u32>) -> Result<(), String> {
    let mut settings = get_settings();
    match delay {
        Some(d) => {
            settings.minimize_delays.insert(item_id.to_string(), d);
        }
        None => {
            settings.minimize_delays.remove(item_id);
        }
    }
    save_settings(&settings)
}

pub fn get_minimize_delay(item_id: &str) -> u32 {
    get_settings().minimize_delays.get(item_id).cloned().unwrap_or(0)
}

pub fn is_auto_exit_enabled() -> bool {
    get_settings().auto_exit_after_minimize
}

pub fn set_auto_exit_enabled(enabled: bool) -> Result<(), String> {
    let mut settings = get_settings();
    settings.auto_exit_after_minimize = enabled;
    save_settings(&settings)
}

pub fn reset_settings() -> Result<(), String> {
    let settings_path = get_settings_path();
    if let Some(parent_dir) = settings_path.parent() {
        if parent_dir.exists() {
            fs::remove_dir_all(parent_dir)
                .map_err(|e| e.to_string())?;
        }
    }

    // 重置全局SETTINGS变量
    let mut guard = SETTINGS.lock().unwrap();
    *guard = AppSettings::default();

    Ok(())
}

/// Record the minimize execution time for an item
pub fn record_minimize_time(item_id: &str) {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);

    let mut guard = MINIMIZE_EXEC_TIMES.lock().unwrap();
    guard.insert(item_id.to_string(), timestamp);
}

/// Get all minimize execution times
pub fn get_all_minimize_times() -> HashMap<String, u64> {
    let guard = MINIMIZE_EXEC_TIMES.lock().unwrap();
    guard.clone()
}

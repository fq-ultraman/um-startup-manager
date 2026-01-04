mod startup;

use startup::{scanner, manager, settings, monitor, StartupItem};
use winreg::enums::*;
use winreg::RegKey;
use tauri::{
    tray::{TrayIconBuilder, MouseButton, MouseButtonState, TrayIconEvent},
    menu::{Menu, MenuItem},
    Manager,
};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_startup_items() -> Vec<StartupItem> {
    let items = scanner::get_all_startup_items();

    // Register all items for potential monitoring
    for item in &items {
        monitor::add_monitored_item(&item.id, &item.path);
    }

    items
}

#[tauri::command]
fn toggle_startup_item(item: StartupItem, enable: bool) -> Result<(), String> {
    manager::toggle_startup_item(&item, enable)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_startup_item(item: StartupItem) -> Result<(), String> {
    manager::delete_startup_item(&item)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_auto_minimize_settings() -> std::collections::HashSet<String> {
    settings::get_settings().auto_minimize_items
}

#[tauri::command]
fn set_auto_minimize(item_id: String, enabled: bool) -> Result<(), String> {
    settings::set_auto_minimize(&item_id, enabled)
}

#[tauri::command]
fn set_process_name_mapping(item_id: String, process_name: Option<String>) -> Result<(), String> {
    settings::set_process_name_mapping(&item_id, process_name)
}

#[tauri::command]
fn get_process_name_mappings() -> std::collections::HashMap<String, String> {
    settings::get_settings().process_name_mappings
}

#[tauri::command]
fn set_minimize_behavior(item_id: String, behavior: String) -> Result<(), String> {
    settings::set_minimize_behavior(&item_id, &behavior)
}

#[tauri::command]
fn get_minimize_behavior(item_id: String) -> String {
    settings::get_minimize_behavior(&item_id)
}

#[tauri::command]
fn set_minimize_delay(item_id: String, delay: Option<u32>) -> Result<(), String> {
    settings::set_minimize_delay(&item_id, delay)
}

#[tauri::command]
fn get_minimize_delay(item_id: String) -> u32 {
    settings::get_minimize_delay(&item_id)
}

#[tauri::command]
fn get_minimize_behaviors() -> std::collections::HashMap<String, String> {
    settings::get_settings().minimize_behaviors
}

#[tauri::command]
fn get_minimize_delays() -> std::collections::HashMap<String, u32> {
    settings::get_settings().minimize_delays
}

#[tauri::command]
fn get_auto_exit_enabled() -> bool {
    settings::is_auto_exit_enabled()
}

#[tauri::command]
fn set_auto_exit_enabled(enabled: bool) -> Result<(), String> {
    settings::set_auto_exit_enabled(enabled)
}

#[tauri::command]
fn reset_settings() -> Result<(), String> {
    startup::settings::reset_settings()
}

#[tauri::command]
fn get_monitor_status() -> (bool, usize) {
    startup::monitor::get_monitor_status()
}

#[tauri::command]
fn get_minimize_exec_times() -> std::collections::HashMap<String, u64> {
    settings::get_all_minimize_times()
}

#[tauri::command]
fn reload_app(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        // 获取当前URL并重新导航，强制刷新页面
        if let Ok(current_url) = window.url() {
            window.navigate(current_url).map_err(|e| e.to_string())?;
        } else {
            // 如果获取URL失败，使用reload方法
            window.reload().map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[tauri::command]
fn start_process_monitor(autostart: bool) {
    monitor::start_monitor(autostart);
}

#[tauri::command]
fn stop_process_monitor() {
    monitor::stop_monitor();
}

const APP_NAME: &str = "UMStartupManager";
const RUN_KEY_PATH: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";

#[tauri::command]
fn get_auto_start_enabled() -> bool {
    // Check HKLM first
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok(key) = hklm.open_subkey(RUN_KEY_PATH) {
        if key.get_value::<String, _>(APP_NAME).is_ok() {
            return true;
        }
    }
    // Then check HKCU
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(key) = hkcu.open_subkey(RUN_KEY_PATH) {
        return key.get_value::<String, _>(APP_NAME).is_ok();
    }
    false
}

#[tauri::command]
fn get_auto_start_priority() -> bool {
    // Check if using HKLM (priority mode)
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok(key) = hklm.open_subkey(RUN_KEY_PATH) {
        return key.get_value::<String, _>(APP_NAME).is_ok();
    }
    false
}

#[tauri::command]
fn set_auto_start_enabled(enabled: bool, priority: bool) -> Result<(), String> {
    let exe_path = std::env::current_exe()
        .map_err(|e| e.to_string())?
        .to_string_lossy()
        .to_string();
    let startup_cmd = format!("\"{}\" --autostart", exe_path);

    if enabled {
        if priority {
            // Try HKLM first
            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
            let result = hklm
                .open_subkey_with_flags(RUN_KEY_PATH, winreg::enums::KEY_SET_VALUE)
                .and_then(|key| key.set_value(APP_NAME, &startup_cmd));

            if result.is_err() {
                // No permission, request elevation
                let args = format!("add \"HKLM\\{}\" /v {} /t REG_SZ /d \"{}\" /f",
                    RUN_KEY_PATH, APP_NAME, startup_cmd);

                let status = std::process::Command::new("powershell")
                    .args(["-Command", &format!(
                        "Start-Process reg -ArgumentList '{}' -Verb RunAs -Wait",
                        args.replace("'", "''")
                    )])
                    .status()
                    .map_err(|e| e.to_string())?;

                if !status.success() {
                    return Err("需要管理员权限".to_string());
                }
            }

            // Remove from HKCU if exists
            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            if let Ok(key) = hkcu.open_subkey_with_flags(RUN_KEY_PATH, winreg::enums::KEY_SET_VALUE) {
                let _ = key.delete_value(APP_NAME);
            }
        } else {
            // Use HKCU
            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            let key = hkcu
                .open_subkey_with_flags(RUN_KEY_PATH, winreg::enums::KEY_SET_VALUE)
                .map_err(|e| e.to_string())?;
            key.set_value(APP_NAME, &startup_cmd)
                .map_err(|e| e.to_string())?;

            // Remove from HKLM if exists (may fail without admin)
            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
            if let Ok(key) = hklm.open_subkey_with_flags(RUN_KEY_PATH, winreg::enums::KEY_SET_VALUE) {
                let _ = key.delete_value(APP_NAME);
            }
        }
    } else {
        // Delete from both locations
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        if let Ok(key) = hkcu.open_subkey_with_flags(RUN_KEY_PATH, winreg::enums::KEY_SET_VALUE) {
            let _ = key.delete_value(APP_NAME);
        }

        // Try to delete from HKLM
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let hklm_result = hklm
            .open_subkey_with_flags(RUN_KEY_PATH, winreg::enums::KEY_SET_VALUE)
            .and_then(|key| key.delete_value(APP_NAME));

        if hklm_result.is_err() {
            // Check if value exists in HKLM
            if let Ok(key) = hklm.open_subkey(RUN_KEY_PATH) {
                if key.get_value::<String, _>(APP_NAME).is_ok() {
                    // Need elevation to delete
                    let args = format!("delete \"HKLM\\{}\" /v {} /f", RUN_KEY_PATH, APP_NAME);
                    let _ = std::process::Command::new("powershell")
                        .args(["-Command", &format!(
                            "Start-Process reg -ArgumentList '{}' -Verb RunAs -Wait",
                            args.replace("'", "''")
                        )])
                        .status();
                }
            }
        }
    }

    Ok(())
}

#[tauri::command]
fn open_registry_location(path: String) -> Result<(), String> {
    // Kill existing regedit process first
    let _ = std::process::Command::new("taskkill")
        .args(["/F", "/IM", "regedit.exe"])
        .output();

    // Set the last key in registry so regedit opens to it
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu
        .create_subkey(r"Software\Microsoft\Windows\CurrentVersion\Applets\Regedit")
        .map_err(|e| e.to_string())?
        .0;
    key.set_value("LastKey", &path).map_err(|e| e.to_string())?;

    // Open regedit
    std::process::Command::new("regedit")
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn open_startup_folder(path: String) -> Result<(), String> {
    std::process::Command::new("explorer")
        .arg(&path)
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn open_file_location(path: String) -> Result<(), String> {
    std::process::Command::new("explorer")
        .args(["/select,", &path])
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn open_config_folder() -> Result<(), String> {
    let app_data = std::env::var("APPDATA").unwrap_or_else(|_| ".".to_string());
    let config_path = std::path::PathBuf::from(app_data)
        .join("UMStartupManager");

    // Ensure the folder exists
    std::fs::create_dir_all(&config_path).map_err(|e| e.to_string())?;

    // Open the folder in explorer
    std::process::Command::new("explorer")
        .arg(config_path)
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn open_services() -> Result<(), String> {
    // Open Windows Services Manager using MMC
    std::process::Command::new("mmc")
        .arg("services.msc")
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn open_task_scheduler() -> Result<(), String> {
    // Open Windows Task Scheduler using MMC
    std::process::Command::new("mmc")
        .arg("taskschd.msc")
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn is_autostart() -> bool {
    std::env::args().any(|arg| arg == "--autostart")
}

#[tauri::command]
fn is_autostart_mode() -> bool {
    is_autostart()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Load settings on startup
    settings::load_settings();

    let autostart = is_autostart();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| {
            // Create tray menu
            let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let show = MenuItem::with_id(app, "show", "显示窗口", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &quit])?;

            // Create tray icon
            // 使用更清晰的图标，直接使用default_window_icon()获取配置的图标
            let _tray = TrayIconBuilder::new()
                // default_window_icon()会自动使用tauri.conf.json中配置的最高质量图标
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| {
                    match event.id.as_ref() {
                        "quit" => {
                            app.exit(0);
                        }
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = event {
                        if let Some(window) = tray.app_handle().get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            // If autostart, hide window to tray
            if autostart {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.hide();
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_startup_items,
            toggle_startup_item,
            delete_startup_item,
            get_auto_minimize_settings,
            set_auto_minimize,
            start_process_monitor,
            stop_process_monitor,
            get_auto_start_enabled,
            get_auto_start_priority,
            set_auto_start_enabled,
            open_registry_location,
            open_startup_folder,
            open_file_location,
            open_config_folder,
            open_services,
            open_task_scheduler,
            set_process_name_mapping,
            get_process_name_mappings,
            set_minimize_behavior,
            get_minimize_behavior,
            set_minimize_delay,
            get_minimize_delay,
            get_minimize_behaviors,
            get_minimize_delays,
            get_auto_exit_enabled,
            set_auto_exit_enabled,
            reset_settings,
            reload_app,
            get_monitor_status,
            get_minimize_exec_times,
            is_autostart_mode
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::path::Path;

#[cfg(windows)]
use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
#[cfg(windows)]
use windows::Win32::System::Threading::{
    OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ,
};
#[cfg(windows)]
use windows::Win32::System::ProcessStatus::GetModuleBaseNameW;
#[cfg(windows)]
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowThreadProcessId, IsWindowVisible, ShowWindow, SW_MINIMIZE,
    PostMessageW, WM_CLOSE,
};

use super::settings::{is_auto_minimize_enabled, mark_as_minimized, was_minimized_this_session, get_process_name_mapping, get_minimize_behavior};

lazy_static::lazy_static! {
    static ref MONITOR_RUNNING: AtomicBool = AtomicBool::new(false);
    /// Maps item_id to process_name (lowercase, without .exe)
    static ref MONITORED_ITEMS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

/// Extract process name from executable path
fn get_process_name_from_path(path: &str) -> String {
    Path::new(path)
        .file_stem()
        .map(|s| s.to_string_lossy().to_lowercase())
        .unwrap_or_default()
}

/// Add an item to monitor
pub fn add_monitored_item(item_id: &str, exe_path: &str) {
    let process_name = get_process_name_from_path(exe_path);
    if !process_name.is_empty() {
        let mut guard = MONITORED_ITEMS.lock().unwrap();
        guard.insert(item_id.to_string(), process_name);
    }
}

/// Get all process names that need to be monitored with their item_ids
fn get_processes_to_monitor() -> Vec<(String, String)> {
    let guard = MONITORED_ITEMS.lock().unwrap();
    guard
        .iter()
        .filter(|(item_id, _)| is_auto_minimize_enabled(item_id))
        .map(|(item_id, default_process_name)| {
            // Use custom mapping if available, otherwise use default
            let process_name = get_process_name_mapping(item_id).unwrap_or_else(|| default_process_name.clone());
            (process_name, item_id.clone())
        })
        .collect()
}

#[cfg(windows)]
fn get_process_name(pid: u32) -> Option<String> {
    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid).ok()?;
        let mut name_buf = [0u16; 260];
        let len = GetModuleBaseNameW(handle, None, &mut name_buf);
        let _ = windows::Win32::Foundation::CloseHandle(handle);

        if len > 0 {
            let name = String::from_utf16_lossy(&name_buf[..len as usize]);
            Some(name.trim_end_matches(".exe").to_lowercase())
        } else {
            None
        }
    }
}

#[cfg(windows)]
struct EnumWindowsData {
    target_processes: Vec<(String, String)>, // (process_name, item_id)
    windows_to_minimize: Vec<(HWND, String)>, // (hwnd, item_id)
}

#[cfg(windows)]
unsafe extern "system" fn enum_windows_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let data = &mut *(lparam.0 as *mut EnumWindowsData);

    if IsWindowVisible(hwnd).as_bool() {
        let mut pid: u32 = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));

        if pid != 0 {
            if let Some(process_name) = get_process_name(pid) {
                for (target_name, item_id) in &data.target_processes {
                    if &process_name == target_name && !was_minimized_this_session(&process_name) {
                        data.windows_to_minimize.push((hwnd, item_id.clone()));
                        break;
                    }
                }
            }
        }
    }

    BOOL(1) // Continue enumeration
}

#[cfg(windows)]
fn find_and_minimize_windows(target_processes: &[(String, String)]) -> Vec<String> {
    let mut minimized = Vec::new();

    unsafe {
        let mut data = EnumWindowsData {
            target_processes: target_processes.to_vec(),
            windows_to_minimize: Vec::new(),
        };

        let _ = EnumWindows(
            Some(enum_windows_callback),
            LPARAM(&mut data as *mut _ as isize),
        );

        // Group windows by process name and minimize/close
        for (hwnd, item_id) in data.windows_to_minimize {
            let mut pid: u32 = 0;
            GetWindowThreadProcessId(hwnd, Some(&mut pid));

            if let Some(process_name) = get_process_name(pid) {
                let behavior = get_minimize_behavior(&item_id);
                if behavior == "close" {
                    let _ = PostMessageW(hwnd, WM_CLOSE, None, None);
                } else {
                    let _ = ShowWindow(hwnd, SW_MINIMIZE);
                }
                if !minimized.contains(&process_name) {
                    minimized.push(process_name);
                }
            }
        }
    }

    minimized
}

#[cfg(not(windows))]
fn find_and_minimize_windows(_target_processes: &[(String, String)]) -> Vec<String> {
    Vec::new()
}

/// Start the background monitor thread
pub fn start_monitor() {
    if MONITOR_RUNNING.swap(true, Ordering::SeqCst) {
        return; // Already running
    }

    thread::spawn(|| {
        while MONITOR_RUNNING.load(Ordering::SeqCst) {
            let processes = get_processes_to_monitor();

            if !processes.is_empty() {
                // Filter out already minimized processes
                let to_check: Vec<(String, String)> = processes
                    .into_iter()
                    .filter(|(p, _)| !was_minimized_this_session(p))
                    .collect();

                if !to_check.is_empty() {
                    let minimized = find_and_minimize_windows(&to_check);
                    for name in minimized {
                        mark_as_minimized(&name);
                    }
                }
            }

            thread::sleep(Duration::from_millis(1000));
        }
    });
}

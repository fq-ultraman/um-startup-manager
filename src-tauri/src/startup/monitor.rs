use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};
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

use super::settings::{is_auto_minimize_enabled, mark_as_minimized, was_minimized_this_session, get_process_name_mapping, get_minimize_behavior, get_minimize_delay, is_auto_exit_enabled, record_minimize_time};

lazy_static::lazy_static! {
    static ref MONITOR_RUNNING: AtomicBool = AtomicBool::new(false);
    /// Whether the application was started via autostart
    static ref IS_AUTOSTART: AtomicBool = AtomicBool::new(false);
    /// Maps item_id to process_name (lowercase, without .exe)
    static ref MONITORED_ITEMS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    /// Maps process_name to (item_id, delay_end_time)
    static ref DELAYED_TASKS: Mutex<HashMap<String, (String, Instant)>> = Mutex::new(HashMap::new());
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

/// Get all process names that need to be monitored with their item_ids and delay times
fn get_processes_to_monitor() -> Vec<(String, String, u32)> {
    let guard = MONITORED_ITEMS.lock().unwrap();
    guard
        .iter()
        .filter(|(item_id, _)| is_auto_minimize_enabled(item_id))
        .map(|(item_id, default_process_name)| {
            // Use custom mapping if available, otherwise use default
            let process_name = get_process_name_mapping(item_id).unwrap_or_else(|| default_process_name.clone());
            let delay = get_minimize_delay(item_id);
            (process_name, item_id.clone(), delay)
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
fn find_and_minimize_windows(target_processes: &[(String, String)]) -> Vec<(String, String)> {
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
                // Add both process_name and item_id to the result
                minimized.push((process_name, item_id));
            }
        }
    }

    minimized
}

#[cfg(not(windows))]
fn find_and_minimize_windows(_target_processes: &[(String, String)]) -> Vec<(String, String)> {
    Vec::new()
}

/// Process delayed tasks that have reached their delay end time
fn process_delayed_tasks() {
    let now = Instant::now();
    let mut tasks_to_remove = Vec::new();
    let mut tasks_to_process = Vec::new();
    
    // Check which delayed tasks have reached their end time
    {
        let mut guard = DELAYED_TASKS.lock().unwrap();
        for (process_name, (item_id, delay_end)) in guard.iter() {
            if now >= *delay_end {
                tasks_to_remove.push(process_name.clone());
                tasks_to_process.push((process_name.clone(), item_id.clone()));
            }
        }
        
        // Remove processed tasks from the map
        for process_name in &tasks_to_remove {
            guard.remove(process_name);
        }
    }
    
    // Process the tasks that have reached their end time
    if !tasks_to_process.is_empty() {
        let results = find_and_minimize_windows(&tasks_to_process);

        // Mark processed processes as minimized and remove from monitoring
        let mut monitor_guard = MONITORED_ITEMS.lock().unwrap();
        for (process_name, item_id) in results {
            mark_as_minimized(&process_name);
            record_minimize_time(&item_id);
            // Remove the item from the monitoring list
            monitor_guard.remove(&item_id);
        }
    }
}

/// Get current monitor status
pub fn get_monitor_status() -> (bool, usize) {
    let running = MONITOR_RUNNING.load(Ordering::SeqCst);
    
    // Count only items that are actually enabled for auto-minimize
    let monitored_count = {
        let guard = MONITORED_ITEMS.lock().unwrap();
        guard.iter()
            .filter(|(item_id, _)| is_auto_minimize_enabled(item_id))
            .count()
    };
    
    (running, monitored_count)
}

/// Stop the background monitor thread
pub fn stop_monitor() {
    MONITOR_RUNNING.store(false, Ordering::SeqCst);
}

/// Start the background monitor thread
pub fn start_monitor(autostart: bool) {
    if MONITOR_RUNNING.swap(true, Ordering::SeqCst) {
        return; // Already running
    }

    // Set the autostart flag
    IS_AUTOSTART.store(autostart, Ordering::SeqCst);

    thread::spawn(|| {
        while MONITOR_RUNNING.load(Ordering::SeqCst) {
            // Process any delayed tasks that have reached their end time
            process_delayed_tasks();

            let processes = get_processes_to_monitor();

            if !processes.is_empty() {
                // Filter out already minimized processes and processes already in delayed tasks
                let to_check: Vec<(String, String, u32)> = processes
                    .into_iter()
                    .filter(|(p, _, _)| !was_minimized_this_session(p))
                    .filter(|(p, _, _)| {
                        let guard = DELAYED_TASKS.lock().unwrap();
                        !guard.contains_key(p)
                    })
                    .collect();

                if !to_check.is_empty() {
                    for (process_name, item_id, delay) in to_check {
                        if delay > 0 {
                            // Add to delayed tasks with end time
                            let delay_end = Instant::now() + Duration::from_secs(delay as u64);
                            DELAYED_TASKS.lock().unwrap().insert(process_name, (item_id, delay_end));
                        } else {
                            // Process immediately
                            let results = find_and_minimize_windows(&[(process_name.clone(), item_id)]);

                            // Mark processed processes as minimized and remove from monitoring
                            let mut monitor_guard = MONITORED_ITEMS.lock().unwrap();
                            for (process_name, item_id) in results {
                                mark_as_minimized(&process_name);
                                record_minimize_time(&item_id);
                                // Remove the item from the monitoring list
                                monitor_guard.remove(&item_id);
                            }
                        }
                    }
                } else {
                    // Check if all processes have been handled and auto-exit is enabled
                    let has_delayed_tasks = {
                        let guard = DELAYED_TASKS.lock().unwrap();
                        !guard.is_empty()
                    };

                    if !has_delayed_tasks && is_auto_exit_enabled() && IS_AUTOSTART.load(Ordering::SeqCst) {
                        // All tasks completed, auto-exit enabled, and running from autostart - exit the application
                        std::process::exit(0);
                    }

                    // No more processes to monitor, stop the monitor thread
                    if !has_delayed_tasks {
                        MONITOR_RUNNING.store(false, Ordering::SeqCst);
                        break;
                    }
                }
            } else {
                // No processes to monitor at all, stop the monitor thread
                let has_delayed_tasks = {
                    let guard = DELAYED_TASKS.lock().unwrap();
                    !guard.is_empty()
                };

                if !has_delayed_tasks {
                    MONITOR_RUNNING.store(false, Ordering::SeqCst);
                    break;
                }
            }

            thread::sleep(Duration::from_millis(1000));
        }
    });
}

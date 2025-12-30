use std::path::PathBuf;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use winreg::enums::*;
use winreg::RegKey;

use super::{StartupItem, SourceType};
use super::icon::extract_icon_base64;

#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;

#[cfg(windows)]
use windows::core::PCWSTR;

/// Get file description from EXE version info
#[cfg(windows)]
pub fn get_file_description(path: &str) -> Option<String> {
    use windows::Win32::Storage::FileSystem::{
        GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW,
    };

    unsafe {
        let wide_path: Vec<u16> = path.encode_utf16().chain(std::iter::once(0)).collect();

        let size = GetFileVersionInfoSizeW(PCWSTR(wide_path.as_ptr()), None);
        if size == 0 {
            return None;
        }

        let mut buffer = vec![0u8; size as usize];

        if GetFileVersionInfoW(
            PCWSTR(wide_path.as_ptr()),
            0,
            size,
            buffer.as_mut_ptr() as *mut _,
        ).is_err() {
            return None;
        }

        // Try common language codes for FileDescription
        let lang_codepages = [
            "040904B0", // US English, Unicode
            "040904E4", // US English, Windows Multilingual
            "080404B0", // Chinese Simplified, Unicode
            "000004B0", // Neutral, Unicode
        ];

        for lang_cp in &lang_codepages {
            let sub_block = format!("\\StringFileInfo\\{}\\FileDescription\0", lang_cp);
            let wide_sub_block: Vec<u16> = sub_block.encode_utf16().collect();

            let mut data_ptr: *mut std::ffi::c_void = std::ptr::null_mut();
            let mut data_len: u32 = 0;

            if VerQueryValueW(
                buffer.as_ptr() as *const _,
                PCWSTR(wide_sub_block.as_ptr()),
                &mut data_ptr,
                &mut data_len,
            ).as_bool() && data_len > 0 {
                let slice = std::slice::from_raw_parts(data_ptr as *const u16, data_len as usize);
                let description = String::from_utf16_lossy(slice)
                    .trim_matches('\0')
                    .to_string();
                if !description.is_empty() {
                    return Some(description);
                }
            }
        }

        // Try to auto-detect language from translation table
        let trans_block = "\\VarFileInfo\\Translation\0";
        let wide_trans: Vec<u16> = trans_block.encode_utf16().collect();

        let mut trans_ptr: *mut std::ffi::c_void = std::ptr::null_mut();
        let mut trans_len: u32 = 0;

        if VerQueryValueW(
            buffer.as_ptr() as *const _,
            PCWSTR(wide_trans.as_ptr()),
            &mut trans_ptr,
            &mut trans_len,
        ).as_bool() && trans_len >= 4 {
            let trans_data = std::slice::from_raw_parts(trans_ptr as *const u16, 2);
            let lang = trans_data[0];
            let codepage = trans_data[1];

            let sub_block = format!(
                "\\StringFileInfo\\{:04X}{:04X}\\FileDescription\0",
                lang, codepage
            );
            let wide_sub_block: Vec<u16> = sub_block.encode_utf16().collect();

            let mut data_ptr: *mut std::ffi::c_void = std::ptr::null_mut();
            let mut data_len: u32 = 0;

            if VerQueryValueW(
                buffer.as_ptr() as *const _,
                PCWSTR(wide_sub_block.as_ptr()),
                &mut data_ptr,
                &mut data_len,
            ).as_bool() && data_len > 0 {
                let slice = std::slice::from_raw_parts(data_ptr as *const u16, data_len as usize);
                let description = String::from_utf16_lossy(slice)
                    .trim_matches('\0')
                    .to_string();
                if !description.is_empty() {
                    return Some(description);
                }
            }
        }

        None
    }
}

#[cfg(not(windows))]
pub fn get_file_description(_path: &str) -> Option<String> {
    None
}

struct RegistrySource {
    hkey: winreg::HKEY,
    path: &'static str,
    name: &'static str,
    is_disabled: bool,
}

const REGISTRY_SOURCES: &[RegistrySource] = &[
    RegistrySource {
        hkey: HKEY_CURRENT_USER,
        path: r"Software\Microsoft\Windows\CurrentVersion\Run",
        name: "HKCU\\...\\Run",
        is_disabled: false,
    },
    RegistrySource {
        hkey: HKEY_CURRENT_USER,
        path: r"Software\Microsoft\Windows\CurrentVersion\RunOnce",
        name: "HKCU\\...\\RunOnce",
        is_disabled: false,
    },
    RegistrySource {
        hkey: HKEY_LOCAL_MACHINE,
        path: r"Software\Microsoft\Windows\CurrentVersion\Run",
        name: "HKLM\\...\\Run",
        is_disabled: false,
    },
    RegistrySource {
        hkey: HKEY_LOCAL_MACHINE,
        path: r"Software\Microsoft\Windows\CurrentVersion\RunOnce",
        name: "HKLM\\...\\RunOnce",
        is_disabled: false,
    },
    // Disabled items
    RegistrySource {
        hkey: HKEY_CURRENT_USER,
        path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run",
        name: "HKCU\\...\\StartupApproved\\Run",
        is_disabled: true,
    },
    RegistrySource {
        hkey: HKEY_LOCAL_MACHINE,
        path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run",
        name: "HKLM\\...\\StartupApproved\\Run",
        is_disabled: true,
    },
];

fn generate_id(source: &str, name: &str) -> String {
    let mut hasher = DefaultHasher::new();
    format!("{}:{}", source, name).hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

fn parse_command_path(command: &str) -> String {
    let command = command.trim();

    // Handle quoted paths
    if command.starts_with('"') {
        if let Some(end_quote) = command[1..].find('"') {
            return command[1..end_quote + 1].to_string();
        }
    }

    // Handle unquoted paths - find first space that's likely separating path from args
    if let Some(exe_pos) = command.to_lowercase().find(".exe") {
        return command[..exe_pos + 4].to_string();
    }

    // Return as-is if we can't parse it
    command.split_whitespace().next().unwrap_or(command).to_string()
}

fn is_item_disabled(hkey: winreg::HKEY, name: &str) -> bool {
    let approval_path = if hkey == HKEY_CURRENT_USER {
        r"Software\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run"
    } else {
        r"Software\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run"
    };

    let root = RegKey::predef(hkey);
    if let Ok(key) = root.open_subkey(approval_path) {
        if let Ok(data) = key.get_raw_value(name) {
            // First byte: 02 = enabled, 03 = disabled
            if !data.bytes.is_empty() && data.bytes[0] == 0x03 {
                return true;
            }
        }
    }
    false
}

pub fn scan_registry_items() -> Vec<StartupItem> {
    let mut items = Vec::new();

    for source in REGISTRY_SOURCES.iter().filter(|s| !s.is_disabled) {
        let root = RegKey::predef(source.hkey);

        if let Ok(key) = root.open_subkey(source.path) {
            for value_result in key.enum_values() {
                if let Ok((name, value)) = value_result {
                    let command = match value.vtype {
                        REG_SZ | REG_EXPAND_SZ => {
                            // Registry strings are UTF-16LE encoded
                            let u16_slice: Vec<u16> = value.bytes
                                .chunks_exact(2)
                                .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                                .collect();
                            String::from_utf16_lossy(&u16_slice)
                                .trim_matches('\0')
                                .to_string()
                        }
                        _ => continue,
                    };

                    if command.is_empty() {
                        continue;
                    }

                    let path = parse_command_path(&command);
                    let enabled = !is_item_disabled(source.hkey, &name);

                    let icon = extract_icon_base64(&path);
                    let description = get_file_description(&path);
                    let valid = std::path::Path::new(&path).exists();

                    let full_source = format!(
                        "{}\\{}",
                        if source.hkey == HKEY_CURRENT_USER { "HKEY_CURRENT_USER" } else { "HKEY_LOCAL_MACHINE" },
                        source.path
                    );

                    items.push(StartupItem {
                        id: generate_id(&full_source, &name),
                        name: name.clone(),
                        description,
                        path: path.clone(),
                        command: command.clone(),
                        icon,
                        source: source.name.to_string(),
                        source_type: SourceType::Registry.to_string(),
                        source_location: full_source,
                        enabled,
                        valid,
                    });
                }
            }
        }
    }

    items
}

fn get_startup_folder_path(all_users: bool) -> Option<PathBuf> {
    if all_users {
        std::env::var("ProgramData")
            .ok()
            .map(|p| PathBuf::from(p).join(r"Microsoft\Windows\Start Menu\Programs\Startup"))
    } else {
        std::env::var("APPDATA")
            .ok()
            .map(|p| PathBuf::from(p).join(r"Microsoft\Windows\Start Menu\Programs\Startup"))
    }
}

#[cfg(windows)]
fn resolve_shortcut(lnk_path: &PathBuf) -> Option<String> {
    use windows::core::{PCWSTR, Interface};
    use windows::Win32::System::Com::{
        CoCreateInstance, CoInitializeEx, CoUninitialize,
        CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED, IPersistFile, STGM_READ,
    };
    use windows::Win32::UI::Shell::{IShellLinkW, ShellLink};
    use windows::Win32::Foundation::MAX_PATH;

    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

        let shell_link: IShellLinkW = match CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER) {
            Ok(link) => link,
            Err(_) => {
                CoUninitialize();
                return None;
            }
        };

        let persist_file: IPersistFile = match shell_link.cast() {
            Ok(pf) => pf,
            Err(_) => {
                CoUninitialize();
                return None;
            }
        };

        let wide_path: Vec<u16> = lnk_path
            .as_os_str()
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        if persist_file.Load(PCWSTR(wide_path.as_ptr()), STGM_READ).is_err() {
            CoUninitialize();
            return None;
        }

        let mut target_path = [0u16; MAX_PATH as usize];

        if shell_link.GetPath(&mut target_path, std::ptr::null_mut(), 0).is_ok() {
            CoUninitialize();
            let path = String::from_utf16_lossy(&target_path);
            let path = path.trim_matches('\0').to_string();
            if !path.is_empty() {
                return Some(path);
            }
        }

        CoUninitialize();
        None
    }
}

#[cfg(not(windows))]
fn resolve_shortcut(_lnk_path: &PathBuf) -> Option<String> {
    None
}

pub fn scan_startup_folder_items() -> Vec<StartupItem> {
    let mut items = Vec::new();

    let folders = [
        (false, "用户启动文件夹"),
        (true, "所有用户启动文件夹"),
    ];

    for (all_users, source_name) in folders.iter() {
        if let Some(folder_path) = get_startup_folder_path(*all_users) {
            if !folder_path.exists() {
                continue;
            }

            if let Ok(entries) = std::fs::read_dir(&folder_path) {
                for entry in entries.flatten() {
                    let file_path = entry.path();
                    let file_name = entry.file_name().to_string_lossy().to_string();

                    // Skip disabled items (with .disabled extension)
                    let (actual_name, enabled) = if file_name.ends_with(".disabled") {
                        (file_name.trim_end_matches(".disabled").to_string(), false)
                    } else {
                        (file_name.clone(), true)
                    };

                    // Skip hidden and system files
                    if actual_name.starts_with('.') {
                        continue;
                    }

                    let (target_path, display_name) = if file_path.extension()
                        .map(|e| e.to_string_lossy().to_lowercase() == "lnk")
                        .unwrap_or(false)
                    {
                        // Resolve .lnk shortcut
                        let target = resolve_shortcut(&file_path).unwrap_or_else(|| file_path.to_string_lossy().to_string());
                        let name = actual_name.trim_end_matches(".lnk").to_string();
                        (target, name)
                    } else if file_path.extension()
                        .map(|e| e.to_string_lossy().to_lowercase() == "exe")
                        .unwrap_or(false)
                    {
                        (file_path.to_string_lossy().to_string(), actual_name.trim_end_matches(".exe").to_string())
                    } else {
                        continue; // Skip non-executable files
                    };

                    let icon = extract_icon_base64(&target_path);
                    let description = get_file_description(&target_path);
                    let source_location = folder_path.to_string_lossy().to_string();
                    let valid = std::path::Path::new(&target_path).exists();

                    items.push(StartupItem {
                        id: generate_id(&source_location, &file_name),
                        name: display_name,
                        description,
                        path: target_path.clone(),
                        command: target_path,
                        icon,
                        source: source_name.to_string(),
                        source_type: SourceType::Folder.to_string(),
                        source_location,
                        enabled,
                        valid,
                    });
                }
            }
        }
    }

    items
}

pub fn get_all_startup_items() -> Vec<StartupItem> {
    let mut items = Vec::new();

    items.extend(scan_registry_items());
    items.extend(scan_startup_folder_items());

    // Sort by name
    items.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    items
}

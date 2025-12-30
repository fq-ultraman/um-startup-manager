use std::path::PathBuf;
use winreg::enums::*;
use winreg::RegKey;

use super::StartupItem;

#[derive(Debug)]
pub enum StartupError {
    NotFound,
    AccessDenied,
    IoError(String),
    RegistryError(String),
}

impl std::fmt::Display for StartupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StartupError::NotFound => write!(f, "启动项未找到"),
            StartupError::AccessDenied => write!(f, "访问被拒绝，可能需要管理员权限"),
            StartupError::IoError(msg) => write!(f, "IO错误: {}", msg),
            StartupError::RegistryError(msg) => write!(f, "注册表错误: {}", msg),
        }
    }
}

fn parse_registry_location(location: &str) -> Option<(winreg::HKEY, String)> {
    if location.starts_with("HKEY_CURRENT_USER\\") {
        Some((HKEY_CURRENT_USER, location.replace("HKEY_CURRENT_USER\\", "")))
    } else if location.starts_with("HKEY_LOCAL_MACHINE\\") {
        Some((HKEY_LOCAL_MACHINE, location.replace("HKEY_LOCAL_MACHINE\\", "")))
    } else {
        None
    }
}

pub fn toggle_registry_item(item: &StartupItem, enable: bool) -> Result<(), StartupError> {
    let (hkey, reg_path) = parse_registry_location(&item.source_location)
        .ok_or(StartupError::NotFound)?;

    let root = RegKey::predef(hkey);

    // Use StartupApproved mechanism (Windows 8+)
    let approval_path = if reg_path.contains("RunOnce") {
        reg_path.replace("RunOnce", "Explorer\\StartupApproved\\RunOnce")
    } else {
        reg_path.replace("Run", "Explorer\\StartupApproved\\Run")
    };

    // Create or open the approval key
    let approval_key = root
        .create_subkey(&approval_path)
        .map_err(|e| StartupError::RegistryError(e.to_string()))?
        .0;

    // Set approval status
    // 02 00 00 00 ... = enabled
    // 03 00 00 00 ... = disabled
    let status_value: Vec<u8> = if enable {
        vec![0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    } else {
        vec![0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    };

    approval_key
        .set_raw_value(
            &item.name,
            &winreg::RegValue {
                bytes: status_value,
                vtype: REG_BINARY,
            },
        )
        .map_err(|e| StartupError::RegistryError(e.to_string()))?;

    Ok(())
}

pub fn toggle_folder_item(item: &StartupItem, enable: bool) -> Result<(), StartupError> {
    let folder_path = PathBuf::from(&item.source_location);

    // Find the actual file
    let entries = std::fs::read_dir(&folder_path)
        .map_err(|e| StartupError::IoError(e.to_string()))?;

    for entry in entries.flatten() {
        let file_name = entry.file_name().to_string_lossy().to_string();
        let file_path = entry.path();

        // Check if this is our item (with or without .disabled suffix)
        let base_name = file_name.trim_end_matches(".disabled");
        let is_our_item = base_name.starts_with(&item.name)
            || base_name.trim_end_matches(".lnk") == item.name
            || base_name.trim_end_matches(".exe") == item.name;

        if is_our_item {
            let is_currently_disabled = file_name.ends_with(".disabled");

            if enable && is_currently_disabled {
                // Remove .disabled suffix
                let new_name = file_name.trim_end_matches(".disabled");
                let new_path = folder_path.join(new_name);
                std::fs::rename(&file_path, &new_path)
                    .map_err(|e| StartupError::IoError(e.to_string()))?;
                return Ok(());
            } else if !enable && !is_currently_disabled {
                // Add .disabled suffix
                let new_name = format!("{}.disabled", file_name);
                let new_path = folder_path.join(new_name);
                std::fs::rename(&file_path, &new_path)
                    .map_err(|e| StartupError::IoError(e.to_string()))?;
                return Ok(());
            } else {
                // Already in desired state
                return Ok(());
            }
        }
    }

    Err(StartupError::NotFound)
}

pub fn toggle_startup_item(item: &StartupItem, enable: bool) -> Result<(), StartupError> {
    if item.source_type == "registry" {
        toggle_registry_item(item, enable)
    } else {
        toggle_folder_item(item, enable)
    }
}

pub fn delete_registry_item(item: &StartupItem) -> Result<(), StartupError> {
    let (hkey, reg_path) = parse_registry_location(&item.source_location)
        .ok_or(StartupError::NotFound)?;

    let root = RegKey::predef(hkey);

    let key = root
        .open_subkey_with_flags(&reg_path, KEY_SET_VALUE)
        .map_err(|e| {
            if e.to_string().contains("Access") {
                StartupError::AccessDenied
            } else {
                StartupError::RegistryError(e.to_string())
            }
        })?;

    key.delete_value(&item.name)
        .map_err(|e| StartupError::RegistryError(e.to_string()))?;

    // Also try to remove from StartupApproved
    let approval_path = if reg_path.contains("RunOnce") {
        reg_path.replace("RunOnce", "Explorer\\StartupApproved\\RunOnce")
    } else {
        reg_path.replace("Run", "Explorer\\StartupApproved\\Run")
    };

    if let Ok(approval_key) = root.open_subkey_with_flags(&approval_path, KEY_SET_VALUE) {
        let _ = approval_key.delete_value(&item.name);
    }

    Ok(())
}

pub fn delete_folder_item(item: &StartupItem) -> Result<(), StartupError> {
    let folder_path = PathBuf::from(&item.source_location);

    let entries = std::fs::read_dir(&folder_path)
        .map_err(|e| StartupError::IoError(e.to_string()))?;

    for entry in entries.flatten() {
        let file_name = entry.file_name().to_string_lossy().to_string();
        let file_path = entry.path();

        let base_name = file_name.trim_end_matches(".disabled");
        let is_our_item = base_name.starts_with(&item.name)
            || base_name.trim_end_matches(".lnk") == item.name
            || base_name.trim_end_matches(".exe") == item.name;

        if is_our_item {
            std::fs::remove_file(&file_path)
                .map_err(|e| StartupError::IoError(e.to_string()))?;
            return Ok(());
        }
    }

    Err(StartupError::NotFound)
}

pub fn delete_startup_item(item: &StartupItem) -> Result<(), StartupError> {
    if item.source_type == "registry" {
        delete_registry_item(item)
    } else {
        delete_folder_item(item)
    }
}

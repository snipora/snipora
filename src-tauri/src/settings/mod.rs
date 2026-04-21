pub mod defaults;
pub mod internal;
pub mod migrations;
pub mod utils;

use std::fs;

use crate::settings::defaults::get_defaults;
use crate::settings::internal::LocalSettings;
use crate::settings::migrations::migrate;
use crate::settings::utils::{deep_merge, get_settings_path};

pub fn load_settings(app: &tauri::AppHandle) -> Result<LocalSettings, String> {
    let path = get_settings_path(app)?;

    if !path.exists() {
        return Ok(get_defaults());
    }

    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;

    let loaded: serde_json::Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;

    let migrated = migrate(loaded)?;

    let defaults_json: serde_json::Value = serde_json::to_value(get_defaults()).map_err(|e| e.to_string())?;

    let merged = deep_merge(defaults_json, migrated);

    let settings: LocalSettings = serde_json::from_value(merged).map_err(|e| e.to_string())?;

    Ok(settings)
}

pub fn save_settings(
    app: &tauri::AppHandle,
    settings: &LocalSettings,
) -> Result<(), String> {
    let path = get_settings_path(app)?;

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
    }

    let content = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;

    // atomic write
    let tmp_path = path.with_added_extension("tmp");
    fs::write(&tmp_path, content).map_err(|e| e.to_string())?;
    fs::rename(&tmp_path, path).map_err(|e| e.to_string())?;

    Ok(())
}

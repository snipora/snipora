use std::path::PathBuf;
use tauri::Manager;

const SETTINGS_FILE_NAME: &str = "settings.local.json";

pub fn get_settings_path(_app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let root = _app.path()
        .app_config_dir()
        .map_err(|e| e.to_string())?;
    if !root.exists() {
        std::fs::create_dir_all(&root).map_err(|e| e.to_string())?;
    }
    let path = root.join(SETTINGS_FILE_NAME);
    Ok(path)
}

pub fn deep_merge(base: serde_json::Value, override_with: serde_json::Value) -> serde_json::Value {
    match (base, override_with) {
        (serde_json::Value::Object(mut base_map), serde_json::Value::Object(override_map)) => {
            for (key, value) in override_map {
                if let Some(base_value) = base_map.get(&key) {
                    base_map.insert(key, deep_merge(base_value.clone(), value));
                } else {
                    base_map.insert(key, value);
                }
            }
            serde_json::Value::Object(base_map)
        }
        (_, override_with) => override_with,
    }
}

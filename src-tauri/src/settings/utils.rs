use std::path::PathBuf;

const SETTINGS_FILE_NAME: &str = "settings.local.json";

#[cfg(debug_assertions)]
pub fn get_settings_path(_app: &tauri::AppHandle) -> Result<PathBuf, String> {
    use std::fs;
    let mut path = PathBuf::from(".dev.storage");
    path.push("config");
    if !path.exists() {
        fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    }
    path.push(SETTINGS_FILE_NAME);
    Ok(path)
}

#[cfg(not(debug_assertions))]
pub fn get_settings_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    use tauri::Manager;
    let path = app
        .path()
        .resolve(SETTINGS_FILE_NAME, tauri::path::BaseDirectory::Config)
        .map_err(|e| e.to_string())?;
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

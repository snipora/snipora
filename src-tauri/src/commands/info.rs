use tauri::{AppHandle, Manager};
use crate::commands::dto::RuntimeInfo;

#[tauri::command]
pub fn runtime_info() -> RuntimeInfo {
    log::debug!("cmd:runtime_info()");
    RuntimeInfo {
        os: std::env::consts::OS.to_lowercase(),
        arch: std::env::consts::ARCH.to_lowercase(),
        bundle_type: tauri::utils::platform::bundle_type()
            .map_or("unknown".into(), |b| b.to_string()),
    }
}

#[tauri::command]
pub fn logs_location(app: AppHandle) -> Result<String, String> {
    log::debug!("cmd:logs_location()");
    let log_dir = app.path().app_log_dir()
        .map_err(|e| e.to_string())?;
    Ok(log_dir.to_string_lossy().into_owned())
}

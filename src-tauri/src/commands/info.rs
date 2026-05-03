use crate::commands::dto::RuntimeInfo;

#[tauri::command]
pub fn runtime_info() -> RuntimeInfo {
    RuntimeInfo {
        os: std::env::consts::OS.to_lowercase(),
        arch: std::env::consts::ARCH.to_lowercase(),
        bundle_type: tauri::utils::platform::bundle_type()
            .map_or("unknown".into(), |b| b.to_string()),
    }
}

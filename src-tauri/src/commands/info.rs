#[tauri::command]
pub fn os_type() -> String {
    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    return "linux".into();
    #[cfg(target_os = "windows")]
    return "windows".into();
    #[cfg(target_os = "macos")]
    return "macos".into();
    #[cfg(target_os = "ios")]
    return "ios".into();
    #[cfg(target_os = "android")]
    return "android".into();
}

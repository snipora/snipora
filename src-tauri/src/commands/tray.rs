use std::str::FromStr;
use crate::tray::tray::TrayIconTheme;

#[tauri::command]
pub fn set_tray_icon(
    app: tauri::AppHandle,
    theme: String,
) -> Result<(), String> {
    let theme = TrayIconTheme::from_str(&theme)?;
    crate::tray::tray::set_tray_icon(&app, theme)?;
    Ok(())
}

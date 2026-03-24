
#[tauri::command]
pub fn set_locale(app: tauri::AppHandle, locale: String) -> Result<(), String> {
    // todo: settings integration - load locale from settings and persist
    rust_i18n::set_locale(&locale);
    crate::tray::tray::rebuild_tray_menu(&app);
    Ok(())
}

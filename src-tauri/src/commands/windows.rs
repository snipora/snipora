#[tauri::command()]
pub fn popup_hide(app_handle: tauri::AppHandle) {
    crate::windows::popup_window::hide(&app_handle);
}

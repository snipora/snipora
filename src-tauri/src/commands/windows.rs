/**
 * Hides the popup window
 */
#[tauri::command()]
pub fn popup_hide(app_handle: tauri::AppHandle) {
    log::debug!("cmd:popup_hide()");
    crate::windows::popup_window::hide(&app_handle);
}

/**
 * Adjusts the window-height of the popup window with a preferred height.
 * The actual new height may differ.
 */
#[tauri::command]
pub fn popup_adjust_height(app_handle: tauri::AppHandle, preferred_height: i32) {
    log::debug!("cmd:popup_adjust_height({:?})", preferred_height);
    crate::windows::popup_window::adjust_height(&app_handle, preferred_height);
}

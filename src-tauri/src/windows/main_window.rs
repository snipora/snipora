use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_window_state::{StateFlags, WindowExt};

fn get_main_window(app: &AppHandle) -> tauri::WebviewWindow {
    app
        .get_webview_window("main")
        .expect("couldn't get main window")
}

pub fn init_main_window(app: &AppHandle) {
    let window = get_main_window(app);
    let app_handle = app.clone();

    window.on_window_event(move |event| {
        if let tauri::WindowEvent::CloseRequested { api, .. } = event {
            api.prevent_close();
            hide(&app_handle);
        }
    });
}

pub fn show_and_focus(app: &AppHandle) {
    let window = get_main_window(app);

    window.unminimize()
        .expect("failed to unminimize main window");
    window.restore_state(StateFlags::POSITION | StateFlags::SIZE | StateFlags::MAXIMIZED)
        .expect("failed to restore main window state");
    window.show()
        .expect("failed to show main window");
    window.set_focus()
        .expect("failed to focus main window");
}

pub fn hide(app: &AppHandle) {
    let window = get_main_window(app);

    window.hide()
        .expect("failed to hide main window");
}

pub fn show_settings(app: &AppHandle) {
    app.emit("main:show-settings", ())
        .expect("failed to emit 'main:show-settings'");
}

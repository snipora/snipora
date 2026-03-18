use std::str::FromStr;
use tauri::{AppHandle, Emitter, Manager, WindowEvent};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};
use tauri_plugin_log::log::warn;


const POPUP_PADDING: f32 = 0.2;  // 20%


fn get_popup_window(app: &AppHandle) -> tauri::WebviewWindow {
    app
        .get_webview_window("popup")
        .expect("couldn't get popup window")
}

pub fn init_popup_window(app: &AppHandle) {
    let window = get_popup_window(app);
    let app_handle = app.clone();

    window.on_window_event(move |event| {
        if let WindowEvent::Focused(false) = event {
            // todo: this fires whenever the shortcut is pressed. need some guard against that.
            hide(&app_handle);
        }
    });

    let shortcut = Shortcut::from_str("Ctrl+Shift+Space")
        .expect("failed to parse shortcut");

    app
        .global_shortcut()
        .on_shortcut(shortcut, |app_handle, _shortcut, _event| {
            show_and_focus(app_handle);
        })
        .expect("failed to register global shortcut");
}

pub fn show_and_focus(app: &AppHandle) {
    let window = get_popup_window(app);

    if !window.is_visible().unwrap_or(false) {
        window.show()
            .expect("failed to show popup window");
    }

    move_to_cursor_monitor(&window);

    if !window.is_focused().unwrap_or(false) {
        window.set_focus()
            .expect("failed to focus popup window");
    }

    window.emit("popup:prepare", ())
        .expect("failed to emit 'popup:prepare'");
    window.emit("popup:focus-input", ())
        .expect("failed to emit 'popup:focus-input'");
}

pub fn hide(app: &AppHandle) {
    let window = get_popup_window(app);

    window.hide()
        .expect("failed to hide popup window");
}

fn move_to_cursor_monitor(window: &tauri::WebviewWindow) {
    if let Ok(cursor) = window.cursor_position() {
        let monitor = window
            .monitor_from_point(cursor.x, cursor.y)
            .ok().flatten()
            .unwrap_or_else(|| {
                warn!("failed to get monitor from point. fallback to primary-monitor");
                window.primary_monitor().ok().flatten()
                    .expect("failed to get primary monitor")
            });

        let monitor_size = monitor.size();
        let monitor_pos = monitor.position();

        let window_size = window.outer_size().unwrap_or_default();

        let x = monitor_pos.x + ((monitor_size.width as i32 - window_size.width as i32) / 2);
        let y = monitor_pos.y + ((monitor_size.height as f32 * POPUP_PADDING) as i32);

        window.set_position(tauri::PhysicalPosition { x, y })
            .expect("failed to set window position");
    } else {
        warn!("failed to get cursor position in order to center window on monitor");
    }
}

pub fn adjust_height(app: &AppHandle, preferred_height: i32) {
    let window = get_popup_window(app);
    let window_width = window.inner_size().unwrap().width as i32;

    let monitor = window
        .current_monitor()
        .ok().flatten()
        .expect("failed to get monitor");

    let monitor_height = monitor.size().height as f32;
    let max_height = (monitor_height * (1. - POPUP_PADDING*2.)) as i32;

    let clamped_height = preferred_height.min(max_height);

    window
        .set_size(tauri::LogicalSize {
            width: window_width,
            height: clamped_height,
        })
        .expect("failed to set window size");
}

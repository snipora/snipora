use std::str::FromStr;
use tauri::{AppHandle, Emitter, Manager, WindowEvent};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};
use crate::settings::internal::LocalSettings;

const POPUP_PADDING: f32 = 0.2; // 20%

fn get_popup_window(app: &AppHandle) -> tauri::WebviewWindow {
    app.get_webview_window("popup")
        .expect("couldn't get popup window")
}

pub fn init_popup_window(app: &AppHandle) {
    log::debug!("init_popup_window");

    let window = get_popup_window(app);
    let app_handle = app.clone();

    let local_settings = app.state::<std::sync::Mutex<LocalSettings>>()
        .lock()
        .expect("failed to lock settings")
        .clone();

    window.on_window_event(move |event| {
        if let WindowEvent::Focused(false) = event {
            // todo: this fires whenever the shortcut is pressed. need some guard against that.
            hide(&app_handle);
        }
    });

    let shortcut = Shortcut::from_str(local_settings.shortcuts.open_popup.as_str())
        .unwrap_or_else(|e| {
            log::error!("failed to parse shortcut {:?} {:?}", local_settings.shortcuts.open_popup, e);
            log::info!("fallback to default shortcut");
            Shortcut::from_str(crate::settings::defaults::get_defaults().shortcuts.open_popup.as_str())
                .expect("failed to create fallback shortcut")
        });

    app.global_shortcut()
        .on_shortcut(shortcut, |app_handle, _shortcut, event| {
            if matches!(event.state, tauri_plugin_global_shortcut::ShortcutState::Pressed) {
                show_and_focus(app_handle);
            }
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

    if window.is_visible().unwrap_or(true) {
        window.hide()
            .expect("failed to hide popup window");
    }
}

fn move_to_cursor_monitor(window: &tauri::WebviewWindow) {
    if let Ok(cursor) = window.cursor_position() {
        let monitor = window
            .monitor_from_point(cursor.x, cursor.y)
            .ok().flatten()
            .unwrap_or_else(|| {
                log::warn!("failed to get monitor from point. fallback to primary-monitor");
                window.primary_monitor()
                    .ok().flatten()
                    .expect("failed to get primary monitor")
            });

        let monitor_size = monitor.size();
        let monitor_pos = monitor.position();

        let window_size = window.outer_size().unwrap_or_default();

        let x = monitor_pos.x + ((monitor_size.width as i32 - window_size.width as i32) / 2);
        let y = monitor_pos.y + ((monitor_size.height as f32 * POPUP_PADDING) as i32);

        log::debug!("window.set_position({:?}, {:?})", x, y);
        window.set_position(tauri::PhysicalPosition { x, y })
            .expect("failed to set window position");
    } else {
        log::warn!("failed to get cursor position in order to center window on monitor");
    }
}

pub fn adjust_height(app: &AppHandle, preferred_height: i32) {
    let window = get_popup_window(app);
    let window_width = window.outer_size().unwrap().width as i32;

    let monitor = window
        .current_monitor()
        .ok().flatten()
        .expect("failed to get monitor");

    let monitor_height = monitor.size().height as f32;
    let max_height = (monitor_height * (1. - POPUP_PADDING * 2.)) as i32;

    let clamped_height = preferred_height.min(max_height);

    log::debug!("window.set_size({:?}, {:?})", window_width, clamped_height);
    window
        .set_size(tauri::PhysicalSize {
            width: window_width,
            height: clamped_height,
        })
        .expect("failed to set window size");
}

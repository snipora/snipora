use std::str::FromStr;
use tauri::{AppHandle, Emitter, Manager, WindowEvent};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};
use crate::settings::internal::LocalSettings;

const POPUP_PADDING: f32 = 0.2; // 20%

fn get_popup_window(app: &AppHandle) -> tauri::WebviewWindow {
    app.get_webview_window("popup")
        .expect("couldn't get popup window")
}

fn get_window_config_width(window: &tauri::WebviewWindow) -> u32 {
    let app = window.app_handle();
    let config = app.config();
    config.app.windows
        .iter()
        .find(|w| w.label == window.label())
        .map(|w| w.width as u32)
        .expect("failed to fallback")
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

    let result = register_global_shortcut(app, local_settings.shortcuts.open_popup.as_str());

    if let Err(error) = result {
        use tauri_plugin_notification::NotificationExt;
        log::error!("failed to register global shortcut for popup ({:?})", error);
        app.notification()
            .builder()
            .title(rust_i18n::t!("popup.shortcut-register-error.title"))
            .body(rust_i18n::t!("popup.shortcut-register-error.body", error = error.to_string()))
            .show()
            .unwrap_or_else(|e| log::error!("{:?}", e));
    }
}

fn unregister_global_shortcut(app: &AppHandle, shortcut: &str) -> Result<(), tauri_plugin_global_shortcut::Error> {
    let shortcut = Shortcut::from_str(shortcut)
        .inspect_err(|e| log::error!("failed to parse {:?} ({:?})", shortcut, e))?;
    app.global_shortcut()
        .unregister(shortcut)?;
    Ok(())
}

fn register_global_shortcut(app: &AppHandle, shortcut: &str) -> Result<(), tauri_plugin_global_shortcut::Error> {
    let shortcut = Shortcut::from_str(shortcut)
        .inspect_err(|e| log::error!("failed to parse {:?} ({:?})", shortcut, e))?;
    app.global_shortcut()
        .on_shortcut(shortcut, shortcut_handle)?;
    Ok(())
}

pub fn change_global_shortcut(app: &AppHandle, old: &str, new: &str) -> Result<(), tauri_plugin_global_shortcut::Error> {
    register_global_shortcut(&app, new)
        .inspect_err(|e| log::error!("failed to register new global shortcut ({:?})", e))?;

    if let Err(error) = unregister_global_shortcut(&app, old) {
        log::error!("failed to unregister old global shortcut ({:?} | {:?})", old, error);

        if let Err(rb_error) = unregister_global_shortcut(&app, new) {
            log::error!("failed rollback of new global shortcut ({:?} | {:?})", new, rb_error);
        }

        return Err(error);
    }

    Ok(())
}

fn shortcut_handle(app_handle: &AppHandle, _shortcut: &Shortcut, event: tauri_plugin_global_shortcut::ShortcutEvent) {
    if matches!(event.state, tauri_plugin_global_shortcut::ShortcutState::Pressed) {
        show_and_focus(app_handle);
    }
}

pub fn show_and_focus(app: &AppHandle) {
    let window = get_popup_window(app);

    move_to_cursor_monitor(&window);

    if !window.is_visible().unwrap_or(false) {
        window.show()
            .expect("failed to show popup window");
    }

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

        let window_width = get_window_config_width(&window) as i32;

        let x = monitor_pos.x + ((monitor_size.width as i32 - window_width) / 2);
        let y = monitor_pos.y + ((monitor_size.height as f32 * POPUP_PADDING) as i32);

        log::debug!("window.set_position({:?}, {:?})", x, y);
        window.set_position(tauri::PhysicalPosition { x, y })
            .expect("failed to set window position");
    } else {
        log::warn!("failed to get cursor position in order to center window on monitor");
    }
}

pub fn adjust_height(app: &AppHandle, preferred_physical_height: i32) {
    let window = get_popup_window(app);
    let window_width = get_window_config_width(&window) as i32;

    let monitor = window
        .current_monitor()
        .ok().flatten()
        .expect("failed to get monitor");

    let monitor_height = monitor.size().height as f32;
    let max_height = (monitor_height * (1. - POPUP_PADDING * 2.)) as i32;

    let clamped_height = preferred_physical_height.min(max_height);

    log::debug!("window.set_size({:?}, {:?})", window_width, clamped_height);
    window
        .set_size(tauri::PhysicalSize {
            width: window_width,
            height: clamped_height,
        })
        .expect("failed to set window size");
}

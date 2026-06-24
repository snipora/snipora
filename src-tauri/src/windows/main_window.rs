use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_window_state::{StateFlags, WindowExt};

#[expect(dead_code)]
#[derive(Clone, serde::Serialize)]
#[serde(tag = "id", rename_all = "kebab-case", rename_all_fields = "camelCase")]
pub enum MainViewState {
    AllSnippets,
    UntaggedSnippets,
    SnippetsByTag { tag: String },
    Settings,
    SnippetById { snippet_id: String },
}

fn get_main_window(app: &AppHandle) -> tauri::WebviewWindow {
    app.get_webview_window("main")
        .expect("couldn't get main window")
}

pub fn init_main_window(app: &AppHandle) {
    log::debug!("init_main_window");
    
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

    if window.is_minimized().unwrap_or(true) {
        window.unminimize()
            .expect("failed to unminimize main window");
    }

    window.restore_state(StateFlags::POSITION | StateFlags::SIZE | StateFlags::MAXIMIZED)
        .expect("failed to restore main window state");

    if !window.is_visible().unwrap_or(false) {
        window.show()
            .expect("failed to show main window");
    }

    if !window.is_focused().unwrap_or(false) {
        window.set_focus()
            .expect("failed to focus main window");
    }
}

pub fn hide(app: &AppHandle) {
    let window = get_main_window(app);

    if window.is_visible().unwrap_or(true) {
        window.hide()
            .expect("failed to hide main window");
    }
}

pub fn set_view_state(app: &AppHandle, state: MainViewState) {
    app.emit("main:set-view-state", state)
        .expect("failed to emit 'main:set-view-state'");
}

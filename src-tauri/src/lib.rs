use tauri_plugin_window_state::StateFlags;

mod windows;
mod tray;
mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::new()
            .with_denylist(&["popup"])
            .with_state_flags(StateFlags::POSITION | StateFlags::SIZE | StateFlags::MAXIMIZED)
            .build()
        )
        .plugin(tauri_plugin_single_instance::init(|app_handle, _args, _cwd| {
            // todo: check for `--from-autostart`?
            windows::main_window::show_and_focus(app_handle);
        }))
        .plugin(tauri_plugin_notification::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_autostart::Builder::new()
            .arg("--from-autostart")
            .build()
        )
        .invoke_handler(tauri::generate_handler![
            commands::windows::popup_hide,
        ])
        .setup(|app| {
            let app_handle = app.handle();
            tray::tray::create_tray(app_handle);
            windows::main_window::init_main_window(app_handle);
            windows::popup_window::init_popup_window(app_handle);
            // todo: change `visible: false` for main-window in `tauri.conf.json`
            // todo: and add check for `--from-autostart` to call `windows::main_window::show_and_focus`
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

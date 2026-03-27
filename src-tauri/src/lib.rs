use tauri::Manager;
use tauri_plugin_window_state::StateFlags;

mod commands;
mod db;
mod tray;
mod windows;

rust_i18n::i18n!("locales");

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
            commands::clipboard::insert_snippet,
            commands::settings::set_locale,
            commands::snippets::create_snippet,
            commands::snippets::update_snippet,
            commands::snippets::delete_snippet,
            commands::snippets::get_all_snippets,
            commands::snippets::get_snippet_by_id,
            commands::snippets::search_snippets,
            commands::tags::get_all_tags,
            commands::tray::set_tray_icon,
            commands::windows::popup_hide,
            commands::windows::popup_adjust_height,
        ])
        .setup(|app| {
            let app_handle = app.handle();
            let conn = db::init_db()
                .expect("failed to init db");
            app.manage(std::sync::Mutex::new(conn));
            tray::tray::create_tray(app_handle);
            windows::main_window::init_main_window(app_handle);
            windows::popup_window::init_popup_window(app_handle);
            // todo: change `visible: false` for main-window in `tauri.conf.json`
            // todo: and add check for `--from-autostart` to call `windows::main_window::show_and_focus`
            windows::main_window::show_and_focus(app_handle);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

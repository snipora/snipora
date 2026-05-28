use tauri::Manager;
use tauri_plugin_log::RotationStrategy;
use tauri_plugin_window_state::StateFlags;

mod commands;
mod db;
mod settings;
mod tray;
mod windows;

rust_i18n::i18n!("locales", fallback = "en");

fn is_from_autostart(args: Vec<String>) -> bool {
    args.iter().any(|arg| arg == "--from-autostart")
}

fn get_logging_level_filter() -> log::LevelFilter {
    if cfg!(debug_assertions) { return log::LevelFilter::Debug; }

    let mut args = std::env::args().into_iter();
    if args.any(|arg| arg == "--debug") {
        return log::LevelFilter::Debug;
    }
    log::LevelFilter::Info
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    log_panics::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::new()
            .with_denylist(&["popup"])
            .with_state_flags(StateFlags::POSITION | StateFlags::SIZE | StateFlags::MAXIMIZED)
            .build()
        )
        .plugin(tauri_plugin_single_instance::init(|app_handle, args, _cwd| {
            if !is_from_autostart(args) {
                windows::main_window::show_and_focus(app_handle);
            }
        }))
        .plugin(tauri_plugin_notification::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(get_logging_level_filter())
                .clear_targets()
                .target(tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stderr))
                .target(tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir { file_name: None }))
                .target(tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Webview))
                .rotation_strategy(RotationStrategy::KeepOne)
                .build(),
        )
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_autostart::Builder::new()
            .arg("--from-autostart")
            .build()
        )
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .invoke_handler(tauri::generate_handler![
            commands::usage::use_snippet,
            commands::info::runtime_info,
            commands::settings::fetch_local_settings,
            commands::settings::update_local_settings,
            commands::snippets::create_snippet,
            commands::snippets::update_snippet,
            commands::snippets::delete_snippet,
            commands::snippets::get_all_snippets,
            commands::snippets::get_snippet_by_id,
            commands::snippets::search_snippets,
            commands::tags::delete_tag,
            commands::tags::rename_tag,
            commands::tags::merge_tag,
            commands::windows::popup_hide,
            commands::windows::popup_adjust_height,
        ])
        .setup(|app| {
            let app_handle = app.handle();

            let pool = tauri::async_runtime::block_on(db::init_db(&app_handle))
                .expect("failed to init db");
            app.manage(pool);

            let clipboard = arboard::Clipboard::new()
                .expect("failed to create clipboard");
            app.manage(std::sync::Mutex::new(clipboard));

            let local_settings = settings::load_settings(&app_handle)
                .expect("failed to load settings");
            rust_i18n::set_locale(&local_settings.general.locale);
            app.set_theme(Some(match local_settings.appearance.ui_theme {
                settings::internal::UiTheme::Dark => tauri::Theme::Dark,
                _ => tauri::Theme::Light,
            }));
            app.manage(std::sync::Mutex::new(local_settings));

            tray::tray::create_tray(app_handle);
            windows::main_window::init_main_window(app_handle);
            windows::popup_window::init_popup_window(app_handle);

            if !is_from_autostart(std::env::args().collect()) {
                windows::main_window::show_and_focus(app_handle);
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

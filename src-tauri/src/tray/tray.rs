use tauri::{AppHandle, Manager};
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use crate::settings::internal::{LocalSettings, TrayIconTheme};

const TRAY_ID: &str = "tray";

fn get_tray(app: &AppHandle) -> tauri::tray::TrayIcon {
    app.tray_by_id(TRAY_ID)
        .expect("failed to get tray")
}

pub fn create_tray(app: &AppHandle) {
    let menu = create_tray_menu(app);
    
    let local_settings = app.state::<std::sync::Mutex<LocalSettings>>()
        .lock()
        .expect("failed to lock settings")
        .clone();
    
    let icon = get_icon_image(app, local_settings.tray.icon_theme)
        .unwrap_or_else(|err| {
            log::warn!("failed to get app-icon: {err}");
            app.default_window_icon().unwrap().clone()
        });

    TrayIconBuilder::with_id(TRAY_ID)
        .icon(icon)
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "open" => {
                crate::windows::main_window::show_and_focus(app);
            }
            "settings" => {
                crate::windows::main_window::show_and_focus(app);
                crate::windows::main_window::show_settings(app);
            }
            "quit" => {
                app.exit(0);
            }
            _ => {
                println!("menu item {:?} not handled", event.id);
            }
        })
        .build(app)
        .expect("failed to create tray icon");
}

fn create_tray_menu(app: &AppHandle) -> Menu<tauri::Wry> {
    Menu::with_items(
        app,
        &[
            &MenuItem::with_id(
                app,
                "open",
                &rust_i18n::t!("tray.open"),
                true, None::<&str>,
            ).unwrap(),
            &MenuItem::with_id(
                app,
                "settings",
                &rust_i18n::t!("tray.settings"),
                true, None::<&str>,
            ).unwrap(),
            &MenuItem::with_id(
                app,
                "quit",
                &rust_i18n::t!("tray.quit"),
                true, None::<&str>,
            ).unwrap(),
        ],
    )
    .expect("failed to create tray-menu")
}

pub fn rebuild_tray_menu(app: &AppHandle) {
    let tray = get_tray(app);
    let menu = create_tray_menu(app);
    if let Err(e) = tray.set_menu(Some(menu)) {
        log::warn!("failed to rebuild tray menu: {e}");
    }
}

pub fn set_tray_icon(app: &AppHandle, theme: TrayIconTheme) -> Result<(), String> {
    let icon = get_icon_image(app, theme)?;

    let tray = get_tray(app);

    tray.set_icon(Some(icon))
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn get_icon_image(
    app: &AppHandle,
    theme: TrayIconTheme,
) -> Result<tauri::image::Image<'static>, String> {
    #[cfg(target_os = "windows")]
    let ext = "ico";
    #[cfg(not(target_os = "windows"))]
    let ext = "png";

    let relative_path = match theme {
        TrayIconTheme::AppIcon => format!("tray-icons/logo-app.{ext}"),
        TrayIconTheme::Light => format!("tray-icons/logo-light.{ext}"),
        TrayIconTheme::Dark => format!("tray-icons/logo-dark.{ext}"),
    };

    let path = app
        .path()
        .resolve(relative_path, tauri::path::BaseDirectory::Resource)
        .map_err(|e| e.to_string())?;

    tauri::image::Image::from_path(path)
        .map_err(|e| e.to_string())
}

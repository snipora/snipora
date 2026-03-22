use tauri::{AppHandle, Manager};
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri_plugin_log::log::warn;

const TRAY_ID: &str = "tray";

fn get_tray(app: &AppHandle) -> tauri::tray::TrayIcon {
    app.tray_by_id(TRAY_ID)
        .expect("failed to get tray")
}

/**
 * Creates the tray-menu.
 */
pub fn create_tray(app: &AppHandle) {
    let menu = Menu::with_items(app, &[
        // todo: I18N
        &MenuItem::with_id(app, "open", "Open Snipora", true, None::<&str>).unwrap(),
        &MenuItem::with_id(app, "settings", "Settings", true, None::<&str>).unwrap(),
        &MenuItem::with_id(app, "quit", "Quit Snipora", true, None::<&str>).unwrap(),
    ])
        .expect("failed to create tray-menu");

    TrayIconBuilder::with_id(TRAY_ID)
        .icon(get_icon_image(app, TrayIconTheme::AppIcon)  // todo: get theme from settings
            .unwrap_or_else(|err| {
                warn!("failed to get app-icon: {err}");
                app.default_window_icon().unwrap().clone()
            }))
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

pub enum TrayIconTheme {
    AppIcon,
    Light,
    Dark,
}

impl std::str::FromStr for TrayIconTheme {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "app" | "app-icon" => Ok(Self::AppIcon),
            "light" => Ok(Self::Light),
            "dark" => Ok(Self::Dark),
            _ => Err(format!("unknown tray icon theme: {s}"))
        }
    }
}

fn get_icon_image(app: &AppHandle, theme: TrayIconTheme) -> Result<tauri::image::Image<'static>, String> {
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

pub fn set_tray_icon(app: &AppHandle, theme: TrayIconTheme) -> Result<(), String> {
    let icon = get_icon_image(app, theme)?;

    let tray = get_tray(app);

    tray.set_icon(Some(icon))
        .map_err(|e| e.to_string())?;

    Ok(())
}

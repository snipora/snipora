use tauri::{AppHandle, Manager};
use tauri::menu::Menu;
use crate::settings::internal::{LocalSettings, TrayIconTheme};
use crate::windows::main_window::MainViewState;

static ICON_APP: tauri::image::Image<'static> = tauri::include_image!("icons/tray/logo/app.png");
static ICON_LIGHT: tauri::image::Image<'static> = tauri::include_image!("icons/tray/logo/light.png");
static ICON_DARK: tauri::image::Image<'static> = tauri::include_image!("icons/tray/logo/dark.png");

const TRAY_ID: &str = "tray";

fn get_tray(app: &AppHandle) -> tauri::tray::TrayIcon {
    app.tray_by_id(TRAY_ID)
        .expect("failed to get tray")
}

pub fn create_tray(app: &AppHandle) {
    log::info!("creating tray");
    let menu = create_tray_menu(app);
    
    let local_settings = app.state::<std::sync::Mutex<LocalSettings>>()
        .lock()
        .expect("failed to lock settings")
        .clone();
    
    let icon = get_icon_image(local_settings.appearance.tray_icon_theme).clone();

    tauri::tray::TrayIconBuilder::with_id(TRAY_ID)
        .icon(icon)
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "open" => {
                log::debug!("opening main-window with all-snippets");
                crate::windows::main_window::show_and_focus(app);
                crate::windows::main_window::set_view_state(app, MainViewState::AllSnippets);
            }
            "settings" => {
                log::debug!("opening main-window with settings");
                crate::windows::main_window::show_and_focus(app);
                crate::windows::main_window::set_view_state(app, MainViewState::Settings);
            }
            "quit" => {
                log::info!("app.exit");
                app.exit(0);
            }
            _ => {
                log::error!("menu item {:?} not handled", event.id);
            }
        })
        .build(app)
        .expect("failed to create tray icon");
}

fn create_tray_menu(app: &AppHandle) -> Menu<tauri::Wry> {
    tauri::menu::MenuBuilder::new(app)
        .text("open", rust_i18n::t!("tray.open"))
        .text("settings", rust_i18n::t!("tray.settings"))
        .separator()
        .text("quit", rust_i18n::t!("tray.quit"))
        .build()
        .expect("failed to build tray-menu")
}

pub fn rebuild_tray_menu(app: &AppHandle) {
    let tray = get_tray(app);
    let menu = create_tray_menu(app);
    if let Err(e) = tray.set_menu(Some(menu)) {
        log::error!("failed to rebuild tray menu: {e}");
    }
}

pub fn set_tray_icon(app: &AppHandle, theme: TrayIconTheme) -> Result<(), String> {
    let icon = get_icon_image(theme).clone();

    let tray = get_tray(app);

    tray.set_icon(Some(icon))
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn get_icon_image(theme: TrayIconTheme) -> &'static tauri::image::Image<'static> {
    match theme {
        TrayIconTheme::AppIcon => &ICON_APP,
        TrayIconTheme::Light => &ICON_LIGHT,
        TrayIconTheme::Dark => &ICON_DARK,
    }
}

use tauri::{AppHandle};
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;

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

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
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

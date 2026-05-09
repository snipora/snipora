use tauri::{Emitter, State};

use crate::commands::dto::PartialLocalSettingsDto;
use crate::settings::{self, internal::LocalSettings};
use crate::tray::tray;

#[tauri::command]
pub fn fetch_local_settings(
    state: State<std::sync::Mutex<LocalSettings>>,
) -> Result<LocalSettings, String> {
    log::debug!("cmd:fetch_local_settings()");
    
    let settings = state.lock()
        .expect("failed to lock settings");
    Ok(settings.clone())
}

#[tauri::command]
pub fn update_local_settings(
    app: tauri::AppHandle,
    state: State<std::sync::Mutex<LocalSettings>>,
    updated_settings: PartialLocalSettingsDto,
) -> Result<(), String> {
    log::debug!("cmd:update_local_settings({:?})", updated_settings);

    let mut current = state.lock()
        .expect("failed to lock settings");

    if let Some(general) = updated_settings.general {
        if let Some(locale) = general.locale {
            current.general.locale = locale.clone();
            rust_i18n::set_locale(&locale);
            tray::rebuild_tray_menu(&app);
        }
        if let Some(snippet_usage_behavior) = general.snippet_usage_behavior {
            current.general.snippet_usage_behavior = snippet_usage_behavior;
        }
    }

    if let Some(shortcuts) = updated_settings.shortcuts {
        if let Some(open_popup) = shortcuts.open_popup {
            current.shortcuts.open_popup = open_popup;
            // todo: clear old shortcut + register new
        }
    }

    if let Some(appearance) = updated_settings.appearance {
        if let Some(show_tag_counts) = appearance.show_tag_counts {
            current.appearance.show_tag_counts = show_tag_counts;
        }
        if let Some(theme) = appearance.ui_theme {
            app.set_theme(Some(match theme {
                settings::internal::UiTheme::Dark => tauri::Theme::Dark,
                _ => tauri::Theme::Light,
            }));
            current.appearance.ui_theme = theme;
        }
        if let Some(icon_theme) = appearance.tray_icon_theme {
            current.appearance.tray_icon_theme = icon_theme.clone();
            let _ = tray::set_tray_icon(&app, icon_theme);
        }
    }

    settings::save_settings(&app, &current)?;

    app.emit("local-settings-changed", current.clone())
        .map_err(|e| e.to_string())?;

    Ok(())
}

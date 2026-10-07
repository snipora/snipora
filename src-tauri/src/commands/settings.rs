use tauri::{Emitter, State};

use crate::commands::dto::PartialLocalSettingsDto;
use crate::settings::{self, internal::LocalSettings};

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
            if let Err(e) = crate::tray::tray::rebuild_tray_menu(&app) {
                log::error!("failed to rebuild tray menu ({:?})", e);
            }
        }
        if let Some(snippet_usage_behavior) = general.snippet_usage_behavior {
            current.general.snippet_usage_behavior = snippet_usage_behavior;
        }
        if let Some(auto_check_for_updates) = general.auto_check_for_updates {
            current.general.auto_check_for_updates = auto_check_for_updates;
        }
    }

    if let Some(shortcuts) = updated_settings.shortcuts {
        if let Some(open_popup) = shortcuts.open_popup {
            if open_popup != current.shortcuts.open_popup {
                if let Err(e) = crate::windows::popup_window::change_global_shortcut(
                    &app, current.shortcuts.open_popup.as_str(), open_popup.as_str(),
                ) {
                    return Err(e.to_string());
                }
                current.shortcuts.open_popup = open_popup;
            }
        }
    }

    if let Some(appearance) = updated_settings.appearance {
        if let Some(show_tag_counts) = appearance.show_tag_counts {
            current.appearance.show_tag_counts = show_tag_counts;
        }
        if let Some(theme) = appearance.ui_theme {
            current.appearance.ui_theme = theme;
        }
        if let Some(icon_theme) = appearance.tray_icon_theme {
            current.appearance.tray_icon_theme = icon_theme.clone();
            if let Err(e) = crate::tray::tray::set_tray_icon(&app, icon_theme) {
                log::error!("failed set tray icon ({:?})", e)
            }
        }
    }

    settings::save_settings(&app, &current)?;

    app.emit("local-settings-changed", current.clone())
        .map_err(|e| e.to_string())?;

    Ok(())
}

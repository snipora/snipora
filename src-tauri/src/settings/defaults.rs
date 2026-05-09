use crate::settings::internal::{GeneralSettings, TrayIconTheme, LocalSettings, ShortcutsSettings, SnippetUsageBehavior, AppearanceSettings, UiTheme};

pub fn get_defaults() -> LocalSettings {
    LocalSettings {
        version: 1,
        general: GeneralSettings {
            locale: sys_locale::get_locale().unwrap_or("en_US".to_string()),
            snippet_usage_behavior: SnippetUsageBehavior::default(),
        },
        shortcuts: ShortcutsSettings {
            open_popup: "CommandOrControl+Shift+Space".to_string(),
        },
        appearance: AppearanceSettings {
            show_tag_counts: true,
            ui_theme: UiTheme::default(),
            tray_icon_theme: TrayIconTheme::default(),
        },
    }
}

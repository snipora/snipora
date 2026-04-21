use crate::settings::internal::{GeneralSettings, TrayIconTheme, LocalSettings, PopupSettings, ShortcutsSettings, SnippetUsageBehavior, TraySettings, UiSettings, UiTheme};

pub fn get_defaults() -> LocalSettings {
    LocalSettings {
        version: 1,
        general: GeneralSettings {
            locale: sys_locale::get_locale().unwrap_or("en_US".to_string()),
        },
        shortcuts: ShortcutsSettings {
            open_popup: "CommandOrControl+Shift+Space".to_string(),
        },
        popup: PopupSettings {
            snippet_usage_behavior: SnippetUsageBehavior::default(),
        },
        ui: UiSettings {
            show_tag_counts: true,
            theme: UiTheme::default(),
        },
        tray: TraySettings {
            icon_theme: TrayIconTheme::default(),
        },
    }
}

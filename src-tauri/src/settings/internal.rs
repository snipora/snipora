use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct LocalSettings {
    pub version: u32,
    pub general: GeneralSettings,
    pub shortcuts: ShortcutsSettings,
    pub popup: PopupSettings,
    pub ui: UiSettings,
    pub tray: TraySettings,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct GeneralSettings {
    pub locale: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ShortcutsSettings {
    #[serde(rename = "openPopup")]
    pub open_popup: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct PopupSettings {
    #[serde(rename = "snippetUsageBehavior")]
    pub snippet_usage_behavior: SnippetUsageBehavior,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct UiSettings {
    #[serde(rename = "showTagCounts")]
    pub show_tag_counts: bool,
    pub theme: UiTheme,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct TraySettings {
    #[serde(rename = "iconTheme")]
    pub icon_theme: TrayIconTheme,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum SnippetUsageBehavior {
    #[default]
    CopyToClipboard,
    SimulatePaste,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum UiTheme {
    #[default]
    System,
    Light,
    Dark,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum TrayIconTheme {
    #[default]
    AppIcon,
    Light,
    Dark,
}

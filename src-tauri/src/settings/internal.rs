use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocalSettings {
    pub version: u32,
    pub general: GeneralSettings,
    pub shortcuts: ShortcutsSettings,
    pub popup: PopupSettings,
    pub ui: UiSettings,
    pub tray: TraySettings,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GeneralSettings {
    pub locale: String,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShortcutsSettings {
    pub open_popup: String,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PopupSettings {
    pub snippet_usage_behavior: SnippetUsageBehavior,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UiSettings {
    pub show_tag_counts: bool,
    pub theme: UiTheme,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TraySettings {
    pub icon_theme: TrayIconTheme,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum SnippetUsageBehavior {
    #[default]
    CopyToClipboard,
    SimulatePaste,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum UiTheme {
    #[default]
    System,
    Light,
    Dark,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum TrayIconTheme {
    #[default]
    AppIcon,
    Light,
    Dark,
}

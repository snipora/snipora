use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocalSettings {
    pub version: u32,
    pub general: GeneralSettings,
    pub shortcuts: ShortcutsSettings,
    pub appearance: AppearanceSettings,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GeneralSettings {
    pub locale: String,
    pub snippet_usage_behavior: SnippetUsageBehavior,
    pub auto_check_for_updates: bool,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShortcutsSettings {
    pub open_popup: String,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppearanceSettings {
    pub show_tag_counts: bool,
    pub ui_theme: UiTheme,
    pub tray_icon_theme: TrayIconTheme,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum SnippetUsageBehavior {
    #[default]
    CopyToClipboard,
    SimulatePaste,
    NaturalTyping,
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

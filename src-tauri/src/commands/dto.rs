use crate::settings::internal::{SnippetUsageBehavior, TrayIconTheme, UiTheme};

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeInfo {
    pub os: String,
    pub arch: String,
    pub bundle_type: String,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnippetDto {
    pub id: String,
    pub label: String,
    pub snippet: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub last_used_at: Option<i64>,
    pub tags: Vec<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct PartialLocalSettingsDto {
    #[serde(default)]
    pub general: Option<PartialGeneralSettings>,
    #[serde(default)]
    pub shortcuts: Option<PartialShortcutsSettings>,
    #[serde(default)]
    pub appearance: Option<PartialAppearanceSettings>,
}

#[derive(serde::Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PartialGeneralSettings {
    #[serde(default)]
    pub locale: Option<String>,
    #[serde(default)]
    pub snippet_usage_behavior: Option<SnippetUsageBehavior>,
    #[serde(default)]
    pub auto_check_for_updates: Option<bool>,
}

#[derive(serde::Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PartialShortcutsSettings {
    #[serde(default)]
    pub open_popup: Option<String>,
}

#[derive(serde::Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PartialAppearanceSettings {
    #[serde(default)]
    pub show_tag_counts: Option<bool>,
    #[serde(default)]
    pub ui_theme: Option<UiTheme>,
    #[serde(default)]
    pub tray_icon_theme: Option<TrayIconTheme>,
}

use crate::settings::internal::{SnippetUsageBehavior, TrayIconTheme, UiTheme};

#[derive(serde::Serialize)]
pub struct RuntimeInfo {
    pub os: String,
    pub arch: String,
    #[serde(rename = "bundleType")]
    pub bundle_type: String,
}

#[derive(serde::Serialize)]
pub struct SnippetDto {
    pub id: String,
    pub label: String,
    pub snippet: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub last_used_at: Option<i64>,
    pub tags: Vec<String>,
}

#[derive(serde::Deserialize)]
pub struct UpdateLocalSettingsDto {
    #[serde(default)]
    pub general: Option<UpdateGeneralSettings>,
    #[serde(default)]
    pub shortcuts: Option<UpdateShortcutsSettings>,
    #[serde(default)]
    pub popup: Option<UpdatePopupSettings>,
    #[serde(default)]
    pub ui: Option<UpdateUiSettings>,
    #[serde(default)]
    pub tray: Option<UpdateTraySettings>,
}

#[derive(serde::Deserialize, Default)]
pub struct UpdateGeneralSettings {
    #[serde(default)]
    pub locale: Option<String>,
}

#[derive(serde::Deserialize, Default)]
pub struct UpdateShortcutsSettings {
    #[serde(default)]
    #[serde(rename = "openPopup")]
    pub open_popup: Option<String>,
}

#[derive(serde::Deserialize, Default)]
pub struct UpdatePopupSettings {
    #[serde(default)]
    #[serde(rename = "snippetUsageBehavior")]
    pub snippet_usage_behavior: Option<SnippetUsageBehavior>,
}

#[derive(serde::Deserialize, Default)]
pub struct UpdateUiSettings {
    #[serde(default)]
    #[serde(rename = "showTagCounts")]
    pub show_tag_counts: Option<bool>,
    #[serde(default)]
    pub theme: Option<UiTheme>,
}

#[derive(serde::Deserialize, Default)]
pub struct UpdateTraySettings {
    #[serde(default)]
    #[serde(rename = "iconTheme")]
    pub icon_theme: Option<TrayIconTheme>,
}

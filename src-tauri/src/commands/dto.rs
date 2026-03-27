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

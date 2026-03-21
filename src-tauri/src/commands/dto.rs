#[derive(serde::Serialize)]
pub struct SnippetDto {
    pub id: String,
    pub label: String,
    pub snippet: String,
    pub tags: Vec<String>,
}

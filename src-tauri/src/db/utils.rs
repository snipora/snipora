use std::path::PathBuf;
use tauri::Manager;

const DATABASE_FILE_NAME: &str = "snipora.db";

pub fn get_database_path(_app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let root = _app.path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&root).map_err(|e| e.to_string())?;
    let path = root.join(DATABASE_FILE_NAME);
    Ok(path)
}

pub fn build_fts_query(input: &str) -> String {
    input
        .split_whitespace()
        .map(|token| format!("\"{}\"*", token.replace("\"", "\"\"")))
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn parse_query(input: &str) -> (String, Vec<String>) {
    let mut terms = vec![];
    let mut tags = vec![];

    for token in input.split_whitespace() {
        if let Some(tag) = token.strip_prefix('@') {
            let tag = tag.trim().to_lowercase();
            if !tag.is_empty() {
                tags.push(tag);
            }
        } else {
            terms.push(token);
        }
    }

    (terms.join(" "), tags)
}

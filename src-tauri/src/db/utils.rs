use std::path::PathBuf;

const DATABASE_FILE_NAME: &str = "snipora.db";

#[cfg(debug_assertions)]
pub fn get_database_path(_app: &tauri::AppHandle) -> Result<PathBuf, String> {
    use std::fs;
    let mut path = PathBuf::from(".dev.storage");
    path.push("app-data");
    if !path.exists() {
        fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    }
    path.push(DATABASE_FILE_NAME);
    Ok(path)
}

#[cfg(not(debug_assertions))]
pub fn get_database_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    use tauri::Manager;
    let path = app
        .path()
        .resolve(DATABASE_FILE_NAME, tauri::path::BaseDirectory::AppData)
        .map_err(|e| e.to_string())?;
    Ok(path)
}

pub fn build_fts_query(input: &str) -> String {
    input
        .split_whitespace()
        .map(|token| format!("{}*", token))
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

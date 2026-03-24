use tauri_plugin_log::log::warn;
use crate::commands::dto::SnippetDto;

#[tauri::command]
pub fn create_snippet(
    state: tauri::State<std::sync::Mutex<rusqlite::Connection>>,
    label: String,
    snippet: String,
    tags: Vec<String>,
) -> Result<String, String> {
    let mut conn = state.lock()
        .expect("failed to get db-conn");
    let tx = conn.transaction()
        .expect("failed to start transaction");

    let id = crate::db::snippets::create_snippet(&tx, label, snippet)
        .map_err(|e| e.to_string())?;

    crate::db::tags::set_snippet_tags(&tx, &id, tags)
        .map_err(|e| e.to_string())?;

    tx.commit()
        .expect("failed to commit transaction");

    Ok(id)
}

#[tauri::command]
pub fn update_snippet(
    state: tauri::State<std::sync::Mutex<rusqlite::Connection>>,
    id: String,
    label: String,
    snippet: String,
    tags: Vec<String>,
) -> Result<(), String> {
    let mut conn = state.lock()
        .expect("failed to get db-conn");
    let tx = conn.transaction()
        .expect("failed to start transaction");

    crate::db::snippets::update_snippet(&tx, &id, label, snippet)
        .map_err(|e| e.to_string())?;

    crate::db::tags::set_snippet_tags(&tx, &id, tags)
        .map_err(|e| e.to_string())?;

    tx.commit()
        .expect("failed to commit transaction");

    Ok(())
}

#[tauri::command]
pub fn delete_snippet(
    state: tauri::State<std::sync::Mutex<rusqlite::Connection>>,
    snippet_id: String,
) -> Result<(), String> {
    let mut conn = state.lock()
        .expect("failed to get db-conn");
    let tx = conn.transaction()
        .expect("failed to start transaction");

    crate::db::snippets::delete_snippet(&tx, &snippet_id)
        .map_err(|e| e.to_string())?;

    // todo: cleanup tags that are no longer used

    tx.commit()
        .expect("failed to commit transaction");

    Ok(())
}

#[tauri::command]
pub fn get_all_snippets(
    state: tauri::State<std::sync::Mutex<rusqlite::Connection>>,
) -> Result<Vec<SnippetDto>, String> {
    let conn = state.lock()
        .expect("failed to get db-conn");

    let snippets = crate::db::snippets::get_all_snippets_with_tags(&conn)
        .map_err(|e| e.to_string())?;

    Ok(snippets.iter().map(|s| {
        SnippetDto {
            id: s.id.clone(),
            label: s.label.clone(),
            snippet: s.snippet.clone(),
            tags: s.tags.clone(),
        }
    }).collect())
}

#[tauri::command]
pub fn get_snippet_by_id(
    state: tauri::State<std::sync::Mutex<rusqlite::Connection>>,
    snippet_id: String,
) -> Result<Option<SnippetDto>, String> {
    let conn = state.lock()
        .expect("failed to get db-conn");
    
    let snippet = crate::db::snippets::get_snippet_with_tags_by_id(&conn, &snippet_id)
        .map_err(|e| e.to_string())?;

    Ok(snippet.map(|s| SnippetDto {
        id: s.id.clone(),
        label: s.label.clone(),
        snippet: s.snippet.clone(),
        tags: s.tags.clone(),
    }))
}

#[tauri::command]
pub fn get_recent_snippets(
    state: tauri::State<std::sync::Mutex<rusqlite::Connection>>,
    limit: i64,
) -> Result<Vec<SnippetDto>, String> {
    let conn = state.lock()
        .expect("failed to get db-conn");

    let recent_snippets = crate::db::snippets::get_recent_snippets(&conn, limit)
        .map_err(|e| e.to_string())?;

    Ok(recent_snippets.iter().map(|s| {
        let tags = crate::db::tags::get_tags_for_snippet(&conn, &s.id)
            .unwrap_or_else(|_| {
                warn!("failed to get tags for snippet {}", &s.id);
                vec![]
            });
        SnippetDto {
            id: s.id.clone(),
            label: s.label.clone(),
            snippet: s.snippet.clone(),
            tags: tags.clone(),
        }
    }).collect())
}

#[tauri::command]
pub fn search_snippets(
    state: tauri::State<std::sync::Mutex<rusqlite::Connection>>,
    query: String,
    limit: i64,
) -> Result<Vec<SnippetDto>, String> {
    let conn = state.lock()
        .expect("failed to get db-conn");
    
    let results = crate::db::snippets::search_snippets(&conn, &query, limit)
        .map_err(|e| e.to_string())?;
    
    Ok(results.into_iter().map(|s| {
        SnippetDto {
            id: s.id.clone(),
            label: s.label.clone(),
            snippet: s.snippet.clone(),
            tags: s.tags.clone(),
        }
    }).collect())
}

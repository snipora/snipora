use crate::commands::dto::SnippetDto;
use crate::commands::utils::emit_data_changed;

#[tauri::command]
pub fn create_snippet(
    app: tauri::AppHandle,
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
    emit_data_changed(app);

    Ok(id)
}

#[tauri::command]
pub fn update_snippet(
    app: tauri::AppHandle,
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
    emit_data_changed(app);

    Ok(())
}

#[tauri::command]
pub fn delete_snippet(
    app: tauri::AppHandle,
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
    emit_data_changed(app);

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

    Ok(snippets.iter().map(|s| SnippetDto {
        id: s.id.clone(),
        label: s.label.clone(),
        snippet: s.snippet.clone(),
        created_at: s.created_at,
        updated_at: s.updated_at,
        last_used_at: s.last_used_at,
        tags: s.tags.clone(),
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
        created_at: s.created_at,
        updated_at: s.updated_at,
        last_used_at: s.last_used_at,
        tags: s.tags.clone(),
    }))
}

#[tauri::command]
pub fn search_snippets(
    state: tauri::State<std::sync::Mutex<rusqlite::Connection>>,
    query: String,
    limit: Option<i64>,
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
            created_at: s.created_at,
            updated_at: s.updated_at,
            last_used_at: s.last_used_at,
            tags: s.tags.clone(),
        }
    }).collect())
}

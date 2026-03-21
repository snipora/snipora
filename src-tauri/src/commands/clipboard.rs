#[tauri::command]
pub fn insert_snippet(
    state: tauri::State<std::sync::Mutex<rusqlite::Connection>>,
    snippet_id: String,
) -> Result<(), String> {
    let mut conn = state.lock()
        .expect("failed to get db-conn");
    let tx = conn.transaction()
        .expect("failed to start transaction");

    let snippet = crate::db::snippets::get_snippet_by_id(&tx, &snippet_id)
        .map_err(|e| e.to_string())?
        .ok_or("snippet not found")?;

    if snippet.snippet.trim().is_empty() {
        return Err("snippet is empty".into());
    }

    use arboard::Clipboard;

    let mut clipboard = Clipboard::new()
        .map_err(|e| e.to_string())?;

    clipboard.set_text(snippet.snippet)
        .map_err(|e| e.to_string())?;

    crate::db::snippets::mark_used(&tx, &snippet_id)
        .map_err(|e| e.to_string())?;

    tx.commit()
        .expect("failed to commit transaction");

    Ok(())
}

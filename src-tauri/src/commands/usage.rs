#[tauri::command]
pub async fn use_snippet(
    pool: tauri::State<'_, sqlx::SqlitePool>,
    state_clipboard: tauri::State<'_, std::sync::Mutex<arboard::Clipboard>>,
    snippet_id: String,
) -> Result<(), String> {
    log::debug!("cmd:insert_snippet({:?})", snippet_id);

    let snippet = crate::db::snippets::get_snippet_by_id(&pool, &snippet_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("snippet not found")?;

    if snippet.snippet.trim().is_empty() {
        return Err("snippet is empty".into());
    }

    {
        let mut clipboard = state_clipboard.lock()
            .expect("failed to get clipboard");
        clipboard.set_text(snippet.snippet)
            .map_err(|e| e.to_string())?;
    }

    let mut tx = pool
        .begin()
        .await
        .map_err(|e| e.to_string())?;

    crate::db::snippets::mark_used(&mut tx, &snippet_id)
        .await
        .map_err(|e| e.to_string())?;

    tx.commit()
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

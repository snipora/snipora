use crate::commands::dto::SnippetDto;
use crate::commands::utils::emit_data_changed;

#[tauri::command]
pub async fn create_snippet(
    app: tauri::AppHandle,
    pool: tauri::State<'_, sqlx::SqlitePool>,
    label: String,
    snippet: String,
    tags: Vec<String>,
) -> Result<String, String> {
    log::debug!("cmd:create_snippet({:?}, {:?}, {:?})", label, snippet, tags);

    let mut tx = pool
        .begin()
        .await
        .map_err(|e| e.to_string())?;

    let id = crate::db::snippets::create_snippet(&mut tx, label, snippet)
        .await
        .map_err(|e| e.to_string())?;

    crate::db::tags::set_snippet_tags(&mut tx, &id, tags)
        .await
        .map_err(|e| e.to_string())?;

    tx.commit()
        .await
        .map_err(|e| e.to_string())?;
    emit_data_changed(app);

    Ok(id)
}

#[tauri::command]
pub async fn update_snippet(
    app: tauri::AppHandle,
    pool: tauri::State<'_, sqlx::SqlitePool>,
    id: String,
    label: String,
    snippet: String,
    tags: Vec<String>,
) -> Result<(), String> {
    log::debug!("cmd::update_snippet({:?}, {:?}, {:?}, {:?})", id, label, snippet, tags);

    let mut tx = pool
        .begin()
        .await
        .map_err(|e| e.to_string())?;

    crate::db::snippets::update_snippet(&mut tx, &id, label, snippet)
        .await
        .map_err(|e| e.to_string())?;

    crate::db::tags::set_snippet_tags(&mut tx, &id, tags)
        .await
        .map_err(|e| e.to_string())?;
    crate::db::tags::cleanup_unused_tags(&mut tx)
        .await
        .map_err(|e| e.to_string())?;

    tx.commit()
        .await
        .map_err(|e| e.to_string())?;
    emit_data_changed(app);

    Ok(())
}

#[tauri::command]
pub async fn delete_snippet(
    app: tauri::AppHandle,
    pool: tauri::State<'_, sqlx::SqlitePool>,
    snippet_id: String,
) -> Result<(), String> {
    log::debug!("cmd:delete_snippet({:?})", snippet_id);

    let mut tx = pool
        .begin()
        .await
        .map_err(|e| e.to_string())?;

    crate::db::snippets::delete_snippet(&mut tx, &snippet_id)
        .await
        .map_err(|e| e.to_string())?;
    crate::db::tags::cleanup_unused_tags(&mut tx)
        .await
        .map_err(|e| e.to_string())?;

    tx.commit()
        .await
        .map_err(|e| e.to_string())?;
    emit_data_changed(app);

    Ok(())
}

#[tauri::command]
pub async fn get_all_snippets(
    pool: tauri::State<'_, sqlx::SqlitePool>,
) -> Result<Vec<SnippetDto>, String> {
    log::debug!("cmd:get_all_snippets()");

    let results = crate::db::snippets::get_all_snippets_with_tags(&pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(results
        .iter()
        .map(|r| SnippetDto {
            id: r.id.clone(),
            label: r.label.clone(),
            snippet: r.snippet.clone(),
            created_at: r.created_at,
            updated_at: r.updated_at,
            last_used_at: r.last_used_at,
            tags: r.tags.clone(),
        })
        .collect())
}

#[tauri::command]
pub async fn get_snippet_by_id(
    pool: tauri::State<'_, sqlx::SqlitePool>,
    snippet_id: String,
) -> Result<Option<SnippetDto>, String> {
    log::debug!("cmd:get_snippet_by_id({:?})", snippet_id);

    let result = crate::db::snippets::get_snippet_with_tags_by_id(&pool, &snippet_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(result.map(|s| SnippetDto {
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
pub async fn search_snippets(
    pool: tauri::State<'_, sqlx::SqlitePool>,
    query: String,
    limit: Option<i64>,
) -> Result<Vec<SnippetDto>, String> {
    log::debug!("cmd:search_snippets({:?})", query);

    let results = crate::db::snippets::search_snippets(&pool, &query, limit)
        .await
        .map_err(|e| e.to_string())?;

    Ok(results
        .into_iter()
        .map(|s| SnippetDto {
            id: s.id.clone(),
            label: s.label.clone(),
            snippet: s.snippet.clone(),
            created_at: s.created_at,
            updated_at: s.updated_at,
            last_used_at: s.last_used_at,
            tags: s.tags.clone(),
        })
        .collect())
}

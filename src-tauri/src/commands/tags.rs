#[tauri::command]
pub async fn delete_tag(
    app: tauri::AppHandle,
    pool: tauri::State<'_, sqlx::SqlitePool>,
    tag: String,
) -> Result<(), String> {
    log::debug!("cmd:delete_tag({:?})", tag);

    let mut tx = pool
        .begin()
        .await
        .map_err(|e| e.to_string())?;

    crate::db::tags::delete_tag(&mut tx, &tag)
        .await
        .map_err(|e| e.to_string())?;

    tx.commit()
        .await
        .map_err(|e| e.to_string())?;

    crate::commands::utils::emit_data_changed(app);

    Ok(())
}

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

#[tauri::command]
pub async fn rename_tag(
    app: tauri::AppHandle,
    pool: tauri::State<'_, sqlx::SqlitePool>,
    tag: String,
    new_name: String,
) -> Result<(), String> {
    log::debug!("cmd:rename_tag({:?}, {:?})", tag, new_name);

    let mut tx = pool
        .begin()
        .await
        .map_err(|e| e.to_string())?;

    crate::db::tags::rename_tag(&mut tx, &tag, &new_name)
        .await
        .map_err(|e| e.to_string())?;

    tx.commit()
        .await
        .map_err(|e| e.to_string())?;

    crate::commands::utils::emit_data_changed(app);

    Ok(())
}

#[tauri::command]
pub async fn merge_tag(
    app: tauri::AppHandle,
    pool: tauri::State<'_, sqlx::SqlitePool>,
    source_tag: String,
    target_tag: String,
) -> Result<(), String> {
    log::debug!("cmd:merge_tag({:?}, {:?})", source_tag, target_tag);

    let mut tx = pool
        .begin()
        .await
        .map_err(|e| e.to_string())?;

    crate::db::tags::merge_tag(&mut tx, &source_tag, &target_tag)
        .await
        .map_err(|e| e.to_string())?;

    tx.commit()
        .await
        .map_err(|e| e.to_string())?;

    crate::commands::utils::emit_data_changed(app);

    Ok(())
}

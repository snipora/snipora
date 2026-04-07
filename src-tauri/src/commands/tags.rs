#[tauri::command]
pub fn delete_tag(
    app: tauri::AppHandle,
    state: tauri::State<std::sync::Mutex<rusqlite::Connection>>,
    tag: String,
) -> Result<(), String> {
    let mut conn = state.lock()
        .expect("failed to get db-conn");

    let tx = conn.transaction()
        .map_err(|e| e.to_string())?;

    crate::db::tags::delete_tag(&tx, &tag)
        .map_err(|e| e.to_string())?;

    tx.commit()
        .map_err(|e| e.to_string())?;

    crate::commands::utils::emit_data_changed(app);

    Ok(())
}

#[tauri::command]
pub fn get_all_tags(
    state: tauri::State<std::sync::Mutex<rusqlite::Connection>>,
) -> Result<Vec<String>, String> {
    let conn = state.lock()
        .expect("failed to get db-conn");

    let snippets = crate::db::tags::get_all_tags(&conn)
        .map_err(|e| e.to_string())?;

    Ok(snippets)
}

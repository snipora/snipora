use rusqlite::Connection;
use crate::db::utils::get_database_path;

pub mod utils;
pub mod schema;
pub mod snippets;
pub mod tags;

pub fn init_db(app_handle: &tauri::AppHandle) -> rusqlite::Result<Connection> {
    let database_path = get_database_path(&app_handle)
        .expect("Failed to get database path");
    let conn = Connection::open(database_path)?;

    conn.execute_batch("PRAGMA foreign_keys = ON;")?;

    schema::init(&conn)?;

    Ok(conn)
}

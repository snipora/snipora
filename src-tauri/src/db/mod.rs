use rusqlite::Connection;

pub mod helper;
pub mod schema;
pub mod snippets;
pub mod tags;

pub fn init_db() -> rusqlite::Result<Connection> {
    let conn = Connection::open("snipora.db")?;

    conn.execute_batch("PRAGMA foreign_keys = ON;")?;

    schema::init(&conn)?;

    Ok(conn)
}

use rusqlite::Connection;

const INIT_SQL: &str = include_str!("../../migrations/0001_init.sql");

pub fn init(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(INIT_SQL)?;

    Ok(())
}

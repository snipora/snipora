use crate::db::utils::get_database_path;

pub mod utils;
pub mod snippets;
pub mod tags;

pub async fn init_db(app_handle: &tauri::AppHandle) -> Result<sqlx::SqlitePool, sqlx::Error> {
    log::debug!("init_db");

    let database_path = get_database_path(app_handle)
        .map_err(|e| sqlx::Error::InvalidArgument(e))?;

    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename(&database_path)
                .create_if_missing(true)
                .foreign_keys(true),
        )
        .await?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    Ok(pool)
}

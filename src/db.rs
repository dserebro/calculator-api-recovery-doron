/// Database pool setup and helper functions.
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Pool, Sqlite};
use std::str::FromStr;

/// Create a SQLite connection pool from the given database URL.
///
/// The pool is configured to create the database file if it doesn't exist.
pub async fn create_pool(database_url: &str) -> Result<Pool<Sqlite>, sqlx::Error> {
    let connect_options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await?;

    Ok(pool)
}

/// Run all pending database migrations.
pub async fn run_migrations(pool: &Pool<Sqlite>) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}

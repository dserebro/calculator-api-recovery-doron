//! Database pool setup and schema bootstrap.
//!
//! Provides SQLite connection pool initialization and table creation.

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::str::FromStr;

/// Shared SQLx SQLite connection pool type.
pub type DbPool = SqlitePool;

/// Initialize the SQLite connection pool and create the schema.
///
/// Creates the `calculations` table if it doesn't already exist.
pub async fn init_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    let options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    // Bootstrap the schema
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS calculations (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            operation   TEXT NOT NULL,
            a           REAL NOT NULL,
            b           REAL NOT NULL,
            result      REAL NOT NULL,
            created_at  TEXT NOT NULL DEFAULT (datetime('now'))
        )
        "#,
    )
    .execute(&pool)
    .await?;

    tracing::info!("Database initialized successfully");
    Ok(pool)
}

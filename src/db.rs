//! Database pool setup and schema initialization.

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::str::FromStr;

/// Type alias for the SQLite connection pool.
pub type DbPool = SqlitePool;

/// SQL statement to create the `calculation` table.
///
/// Table name is `calculation` to match the Python SQLModel default (lowercase class name).
const CREATE_TABLE_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS calculation (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    operation   TEXT NOT NULL,
    a           REAL NOT NULL,
    b           REAL NOT NULL,
    result      REAL NOT NULL,
    created_at  TEXT NOT NULL DEFAULT (datetime('now'))
);
"#;

/// Initialize the SQLite connection pool and create the schema.
///
/// # Arguments
///
/// * `database_url` - SQLite connection URL (e.g., `sqlite:calculator.db` or `sqlite::memory:`)
///
/// # Returns
///
/// A configured `SqlitePool` with the schema already created.
pub async fn init_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    let options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    // Run schema bootstrap
    sqlx::query(CREATE_TABLE_SQL).execute(&pool).await?;

    tracing::info!("Database initialized successfully");
    Ok(pool)
}

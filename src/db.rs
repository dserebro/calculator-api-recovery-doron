//! Database pool setup and initialization.
//!
//! Provides SQLite connection pool creation and table bootstrap.

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::str::FromStr;

/// Shared database pool type alias.
pub type DbPool = SqlitePool;

/// SQL to create the calculations table if it does not exist.
const CREATE_TABLE_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS calculations (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    operation   TEXT NOT NULL,
    a           REAL NOT NULL,
    b           REAL NOT NULL,
    result      REAL NOT NULL,
    created_at  TEXT NOT NULL DEFAULT (datetime('now'))
);
"#;

/// Initialize the SQLite connection pool and ensure the schema exists.
///
/// Creates the database file if it doesn't exist (SQLite default behavior)
/// and runs the table creation SQL.
pub async fn init_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    let options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    // Bootstrap the schema
    sqlx::query(CREATE_TABLE_SQL).execute(&pool).await?;

    Ok(pool)
}

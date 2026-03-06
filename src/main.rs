//! Calculator API - Entrypoint
//!
//! A simple calculator REST API server, translated from Python/FastAPI to Rust/Axum.

use calculator_api::{db, routes};

use std::env;
use std::net::SocketAddr;

use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    // Load .env file if present (ignored if missing).
    dotenvy::dotenv().ok();

    // Initialize structured logging with tracing.
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    // Read configuration from environment variables with sensible defaults.
    let database_url =
        env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:calculator.db".to_string());
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = env::var("SERVER_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8000);

    // Initialize the database pool and ensure schema exists.
    let pool = db::init_pool(&database_url)
        .await
        .expect("Failed to initialize database pool");

    tracing::info!("Database initialized at {}", database_url);

    // Build the Axum router with all routes.
    let app = routes::create_router(pool);

    // Bind and serve.
    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid server address");
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    tracing::info!("Calculator API server listening on {}", addr);

    axum::serve(listener, app)
        .await
        .expect("Server error");
}

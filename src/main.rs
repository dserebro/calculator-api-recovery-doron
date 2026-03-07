//! Calculator API — Entrypoint
//!
//! A Rust translation of the Python FastAPI calculator service.

use calculator_api::{db, routes};
use std::env;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Load .env file if present (ignore errors if missing)
    dotenvy::dotenv().ok();

    // Initialize tracing/logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    // Read configuration from environment variables with defaults
    let database_url =
        env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:calculator.db".to_string());
    let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let server_port = env::var("SERVER_PORT").unwrap_or_else(|_| "8000".to_string());

    // Initialize database pool
    let pool = db::init_pool(&database_url)
        .await
        .expect("Failed to initialize database pool");

    // Build the Axum router
    let app = routes::create_router(pool);

    // Start the server
    let bind_addr = format!("{server_host}:{server_port}");
    tracing::info!("Starting server on {}", bind_addr);

    let listener = TcpListener::bind(&bind_addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}

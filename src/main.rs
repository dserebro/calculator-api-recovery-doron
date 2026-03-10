//! Calculator API - Rust implementation.
//!
//! A REST API server providing arithmetic operations and calculation history,
//! translated from the Python FastAPI implementation.

mod core;
mod db;
mod models;
mod routes;

use std::env;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Load .env file if present
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
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "8000".to_string());

    // Initialize database pool
    let pool = db::init_pool(&database_url)
        .await
        .expect("Failed to initialize database");

    // Build the router
    let app = routes::create_router(pool);

    // Bind and serve
    let addr = format!("{}:{}", host, port);
    tracing::info!("Starting server on {}", addr);

    let listener = TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}

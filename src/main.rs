/// Calculator API — Application entrypoint, configuration, and server startup.
mod core;
mod db;
mod models;
pub mod routes;

use actix_web::{web, App, HttpServer};
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    // Parse configuration from environment variables with defaults
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:calculator.db?mode=rwc".to_string());

    let host = std::env::var("APP_HOST")
        .unwrap_or_else(|_| "0.0.0.0".to_string());

    let port: u16 = std::env::var("APP_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8000);

    // Initialize database connection pool
    let pool = db::create_pool(&database_url)
        .await
        .expect("Failed to create database pool");

    // Run database migrations
    db::run_migrations(&pool)
        .await
        .expect("Failed to run database migrations");

    info!("Starting Calculator API on {}:{}", host, port);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::configure)
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}

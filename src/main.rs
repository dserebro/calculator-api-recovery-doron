/// Calculator API - A simple calculator REST API.
///
/// Rust implementation of the FastAPI calculator service using Actix-web.
mod core;
mod db;
mod models;
pub mod routes;

use actix_web::{App, HttpServer, middleware};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    // Parse configuration from environment with defaults
    let host = std::env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

    let port: u16 = std::env::var("APP_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8000);

    log::info!("Starting Calculator API on {}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(routes::configure_routes)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

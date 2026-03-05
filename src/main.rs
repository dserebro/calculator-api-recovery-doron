mod core;
mod models;
mod routes;

use actix_web::{App, HttpServer};
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let host = std::env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = std::env::var("APP_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8000);

    info!("Starting server on {}:{}", host, port);

    HttpServer::new(|| App::new().configure(routes::configure))
        .bind((host.as_str(), port))?
        .run()
        .await
}

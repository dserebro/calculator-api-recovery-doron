mod core;
mod errors;
mod models;
mod routes;

use actix_web::{App, HttpServer};
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let host = std::env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = std::env::var("APP_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8000);

    info!("Starting calculator-api on {}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .configure(routes::configure)
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}

/// Build an `App` instance for use in integration tests.
pub fn create_app() -> App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new().configure(routes::configure)
}

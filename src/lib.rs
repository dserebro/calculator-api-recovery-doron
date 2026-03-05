/// Calculator API library crate.
///
/// Re-exports modules for use by integration tests and external consumers.
pub mod core;
pub mod db;
pub mod models;
pub mod routes;

use actix_web::web;

/// Returns a JsonConfig that returns 422 Unprocessable Entity for deserialization errors,
/// matching FastAPI's validation error behavior.
pub fn json_error_config() -> web::JsonConfig {
    web::JsonConfig::default().error_handler(|err, _req| {
        let detail = format!("{}", err);
        let response = actix_web::HttpResponse::UnprocessableEntity().json(
            models::ErrorDetail { detail },
        );
        actix_web::error::InternalError::from_response(err, response).into()
    })
}

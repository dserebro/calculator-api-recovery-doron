/// HTTP route definitions and handlers for the Calculator API.
use actix_web::{web, HttpResponse};

use crate::core;
use crate::models::{ApiError, CalculationRequest, HealthResponse, ResultResponse};

/// Health check handler.
///
/// Returns the service status and version.
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok".to_string(),
        version: "0.1.0".to_string(),
    })
}

/// Add two numbers.
pub async fn add(req: web::Json<CalculationRequest>) -> HttpResponse {
    let result = core::add(req.a, req.b);
    HttpResponse::Ok().json(ResultResponse { result })
}

/// Subtract b from a.
pub async fn subtract(req: web::Json<CalculationRequest>) -> HttpResponse {
    let result = core::subtract(req.a, req.b);
    HttpResponse::Ok().json(ResultResponse { result })
}

/// Multiply two numbers.
pub async fn multiply(req: web::Json<CalculationRequest>) -> HttpResponse {
    let result = core::multiply(req.a, req.b);
    HttpResponse::Ok().json(ResultResponse { result })
}

/// Divide a by b.
///
/// Returns HTTP 400 if b is zero.
pub async fn divide(
    req: web::Json<CalculationRequest>,
) -> Result<HttpResponse, ApiError> {
    match core::divide(req.a, req.b) {
        Ok(result) => Ok(HttpResponse::Ok().json(ResultResponse { result })),
        Err(msg) => Err(ApiError::BadRequest(msg)),
    }
}

/// Configure all routes for the application.
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/health").route(web::get().to(health_check)),
    )
    .service(
        web::resource("/add").route(web::post().to(add)),
    )
    .service(
        web::resource("/subtract").route(web::post().to(subtract)),
    )
    .service(
        web::resource("/multiply").route(web::post().to(multiply)),
    )
    .service(
        web::resource("/divide").route(web::post().to(divide)),
    );
}

/// HTTP route definitions and handlers for the calculator API.
///
/// Stateless endpoints for health check and arithmetic operations.
use actix_web::{web, HttpResponse};

use crate::core;
use crate::models::{AppError, CalculationRequest, HealthResponse, ResultResponse};

/// Health check handler.
///
/// Returns service status and version.
pub async fn health_check() -> HttpResponse {
    let response = HealthResponse {
        status: "ok".to_string(),
        version: "0.1.0".to_string(),
    };
    HttpResponse::Ok().json(response)
}

/// Add two numbers.
pub async fn api_add(req: web::Json<CalculationRequest>) -> HttpResponse {
    let result = core::add(req.a, req.b);
    HttpResponse::Ok().json(ResultResponse { result })
}

/// Subtract b from a.
pub async fn api_subtract(req: web::Json<CalculationRequest>) -> HttpResponse {
    let result = core::subtract(req.a, req.b);
    HttpResponse::Ok().json(ResultResponse { result })
}

/// Multiply two numbers.
pub async fn api_multiply(req: web::Json<CalculationRequest>) -> HttpResponse {
    let result = core::multiply(req.a, req.b);
    HttpResponse::Ok().json(ResultResponse { result })
}

/// Divide a by b.
///
/// Returns HTTP 400 with `{"detail": "Cannot divide by zero"}` when b is zero.
pub async fn api_divide(
    req: web::Json<CalculationRequest>,
) -> Result<HttpResponse, AppError> {
    match core::divide(req.a, req.b) {
        Ok(result) => Ok(HttpResponse::Ok().json(ResultResponse { result })),
        Err(msg) => Err(AppError::BadRequest(msg)),
    }
}

/// Configure routes for the application.
///
/// This is the reusable app factory function that registers all routes.
/// Used by both main.rs for production and integration tests.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check))
        .route("/add", web::post().to(api_add))
        .route("/subtract", web::post().to(api_subtract))
        .route("/multiply", web::post().to(api_multiply))
        .route("/divide", web::post().to(api_divide));
}

/// HTTP route definitions and handlers for the Calculator API.
use actix_web::{web, HttpResponse, http::StatusCode};

use crate::core;
use crate::models::{ApiError, CalculationRequest, ErrorDetail, HealthResponse, ResultResponse};

/// Helper to parse JSON body, returning 422 on deserialization errors (matching FastAPI behavior).
fn parse_json<T: serde::de::DeserializeOwned>(body: &str) -> Result<T, HttpResponse> {
    serde_json::from_str(body).map_err(|e| {
        HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY)
            .json(ErrorDetail {
                detail: format!("Json deserialize error: {}", e),
            })
    })
}

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
pub async fn add(body: String) -> HttpResponse {
    let req: CalculationRequest = match parse_json(&body) {
        Ok(r) => r,
        Err(resp) => return resp,
    };
    let result = core::add(req.a, req.b);
    HttpResponse::Ok().json(ResultResponse { result })
}

/// Subtract b from a.
pub async fn subtract(body: String) -> HttpResponse {
    let req: CalculationRequest = match parse_json(&body) {
        Ok(r) => r,
        Err(resp) => return resp,
    };
    let result = core::subtract(req.a, req.b);
    HttpResponse::Ok().json(ResultResponse { result })
}

/// Multiply two numbers.
pub async fn multiply(body: String) -> HttpResponse {
    let req: CalculationRequest = match parse_json(&body) {
        Ok(r) => r,
        Err(resp) => return resp,
    };
    let result = core::multiply(req.a, req.b);
    HttpResponse::Ok().json(ResultResponse { result })
}

/// Divide a by b.
///
/// Returns HTTP 400 if b is zero.
pub async fn divide(body: String) -> Result<HttpResponse, ApiError> {
    let req: CalculationRequest = match parse_json(&body) {
        Ok(r) => r,
        Err(resp) => return Ok(resp),
    };
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

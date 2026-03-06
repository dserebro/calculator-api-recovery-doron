//! HTTP route handlers and router construction.
//!
//! Implements all HTTP endpoints matching the Python FastAPI server.

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use crate::core;
use crate::db::DbPool;
use crate::models::{CalculationResult, ErrorResponse, HealthResponse, Operands};

/// Build the Axum router with all endpoint routes.
///
/// The router is configured with the database pool as shared state,
/// which is used by CRUD endpoints (added in Milestone 2).
pub fn create_router(pool: DbPool) -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .route("/add", post(add_handler))
        .route("/subtract", post(subtract_handler))
        .route("/multiply", post(multiply_handler))
        .route("/divide", post(divide_handler))
        .with_state(pool)
}

/// GET /health — Returns service health status.
///
/// Response: `{"status": "ok", "version": "0.1.0"}` with HTTP 200.
async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: "0.1.0".to_string(),
    })
}

/// POST /add — Add two numbers.
///
/// Request: `{"a": f64, "b": f64}`
/// Response: `{"result": f64}` with HTTP 200.
async fn add_handler(Json(payload): Json<Operands>) -> Json<CalculationResult> {
    Json(CalculationResult {
        result: core::add(payload.a, payload.b),
    })
}

/// POST /subtract — Subtract b from a.
///
/// Request: `{"a": f64, "b": f64}`
/// Response: `{"result": f64}` with HTTP 200.
async fn subtract_handler(Json(payload): Json<Operands>) -> Json<CalculationResult> {
    Json(CalculationResult {
        result: core::subtract(payload.a, payload.b),
    })
}

/// POST /multiply — Multiply two numbers.
///
/// Request: `{"a": f64, "b": f64}`
/// Response: `{"result": f64}` with HTTP 200.
async fn multiply_handler(Json(payload): Json<Operands>) -> Json<CalculationResult> {
    Json(CalculationResult {
        result: core::multiply(payload.a, payload.b),
    })
}

/// POST /divide — Divide a by b.
///
/// Request: `{"a": f64, "b": f64}`
/// Response: `{"result": f64}` with HTTP 200.
/// Error: HTTP 400 with `{"detail": "Cannot divide by zero"}` when b is 0.
async fn divide_handler(
    Json(payload): Json<Operands>,
) -> Result<Json<CalculationResult>, impl IntoResponse> {
    match core::divide(payload.a, payload.b) {
        Ok(result) => Ok(Json(CalculationResult { result })),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                detail: e.to_string(),
            }),
        )),
    }
}

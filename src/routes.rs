//! HTTP route handlers and router construction.
//!
//! Defines the Axum router with all endpoint handlers for the calculator API.

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};

use crate::core;
use crate::db::DbPool;
use crate::models::{ApiError, HealthResponse, Operands, ResultResponse};

/// Build the Axum router with all routes and shared state.
pub fn create_router(pool: DbPool) -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .route("/add", post(add_handler))
        .route("/subtract", post(subtract_handler))
        .route("/multiply", post(multiply_handler))
        .route("/divide", post(divide_handler))
        .with_state(pool)
}

/// GET /health - Health check endpoint.
async fn health_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(HealthResponse {
            status: "ok".to_string(),
            version: "0.1.0".to_string(),
        }),
    )
}

/// POST /add - Add two numbers.
async fn add_handler(
    Json(payload): Json<Operands>,
) -> impl IntoResponse {
    let result = core::add(payload.a, payload.b);
    (StatusCode::OK, Json(ResultResponse { result }))
}

/// POST /subtract - Subtract b from a.
async fn subtract_handler(
    Json(payload): Json<Operands>,
) -> impl IntoResponse {
    let result = core::subtract(payload.a, payload.b);
    (StatusCode::OK, Json(ResultResponse { result }))
}

/// POST /multiply - Multiply two numbers.
async fn multiply_handler(
    Json(payload): Json<Operands>,
) -> impl IntoResponse {
    let result = core::multiply(payload.a, payload.b);
    (StatusCode::OK, Json(ResultResponse { result }))
}

/// POST /divide - Divide a by b.
///
/// Returns 400 Bad Request if b is zero.
async fn divide_handler(
    Json(payload): Json<Operands>,
) -> Result<impl IntoResponse, ApiError> {
    match core::divide(payload.a, payload.b) {
        Ok(result) => Ok((StatusCode::OK, Json(ResultResponse { result }))),
        Err(e) => Err(ApiError::bad_request(e.to_string())),
    }
}

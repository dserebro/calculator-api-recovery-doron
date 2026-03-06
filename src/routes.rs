//! HTTP route handlers and router construction.

use axum::{
    routing::{get, post},
    Json, Router,
};

use crate::core;
use crate::db::DbPool;
use crate::models::{
    AppError, CalculationRequest, HealthResponse, ResultResponse,
};

/// Application state shared across all handlers.
/// The pool field will be used in Milestone 2 for CRUD endpoint database access.
#[derive(Clone)]
#[allow(dead_code)]
pub struct AppState {
    pub pool: DbPool,
}

/// Build the Axum router with all routes.
pub fn create_router(pool: DbPool) -> Router {
    let state = AppState { pool };

    Router::new()
        .route("/health", get(health_handler))
        .route("/add", post(add_handler))
        .route("/subtract", post(subtract_handler))
        .route("/multiply", post(multiply_handler))
        .route("/divide", post(divide_handler))
        .with_state(state)
}

/// GET /health — Returns service health status.
async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: "0.1.0".to_string(),
    })
}

/// POST /add — Add two numbers.
async fn add_handler(
    Json(payload): Json<CalculationRequest>,
) -> Json<ResultResponse> {
    Json(ResultResponse {
        result: core::add(payload.a, payload.b),
    })
}

/// POST /subtract — Subtract b from a.
async fn subtract_handler(
    Json(payload): Json<CalculationRequest>,
) -> Json<ResultResponse> {
    Json(ResultResponse {
        result: core::subtract(payload.a, payload.b),
    })
}

/// POST /multiply — Multiply two numbers.
async fn multiply_handler(
    Json(payload): Json<CalculationRequest>,
) -> Json<ResultResponse> {
    Json(ResultResponse {
        result: core::multiply(payload.a, payload.b),
    })
}

/// POST /divide — Divide a by b.
///
/// Returns HTTP 400 if b is zero, matching the Python app's error behavior.
async fn divide_handler(
    Json(payload): Json<CalculationRequest>,
) -> Result<Json<ResultResponse>, AppError> {
    match core::divide(payload.a, payload.b) {
        Ok(result) => Ok(Json(ResultResponse { result })),
        Err(e) => Err(AppError::BadRequest(e.to_string())),
    }
}

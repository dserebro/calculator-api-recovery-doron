//! Request/response DTOs and database model structs.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

// --- Request DTOs ---

/// Request body for arithmetic endpoints (POST /add, /subtract, /multiply, /divide).
#[derive(Debug, Deserialize)]
pub struct CalculationRequest {
    pub a: f64,
    pub b: f64,
}

/// Request body for creating a calculation (POST /calculations).
/// Used in Milestone 2 for CRUD endpoints.
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct CreateCalculationRequest {
    pub operation: String,
    pub a: f64,
    pub b: f64,
}

// --- Response DTOs ---

/// Response body for the health check endpoint.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

/// Response body for arithmetic endpoints.
#[derive(Debug, Serialize)]
pub struct ResultResponse {
    pub result: f64,
}

/// Response body for calculation CRUD endpoints.
/// Used in Milestone 2 for CRUD endpoints.
#[allow(dead_code)]
#[derive(Debug, Serialize, Clone)]
pub struct CalculationResponse {
    pub id: i64,
    pub operation: String,
    pub a: f64,
    pub b: f64,
    pub result: f64,
    pub created_at: String,
}

// --- Error Response ---

/// Error response matching FastAPI's `{"detail": "<message>"}` format.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub detail: String,
}

/// Application error type that maps to HTTP error responses.
#[derive(Debug)]
pub enum AppError {
    /// 400 Bad Request with a detail message.
    BadRequest(String),
    /// 404 Not Found with a detail message.
    /// Used in Milestone 2 for CRUD endpoints.
    #[allow(dead_code)]
    NotFound(String),
    /// 500 Internal Server Error with a detail message.
    /// Used in Milestone 2 for error handling.
    #[allow(dead_code)]
    InternalError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, detail) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        let body = Json(ErrorResponse { detail });
        (status, body).into_response()
    }
}

// --- Database Model ---

/// Database row struct for the `calculation` table.
/// Used in Milestone 2 for CRUD endpoints.
#[allow(dead_code)]
#[derive(Debug, sqlx::FromRow)]
pub struct Calculation {
    pub id: i64,
    pub operation: String,
    pub a: f64,
    pub b: f64,
    pub result: f64,
    pub created_at: String,
}

impl From<Calculation> for CalculationResponse {
    fn from(calc: Calculation) -> Self {
        CalculationResponse {
            id: calc.id,
            operation: calc.operation,
            a: calc.a,
            b: calc.b,
            result: calc.result,
            created_at: calc.created_at,
        }
    }
}

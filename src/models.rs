//! Request/response DTOs and error types.
//!
//! Defines the data structures used for HTTP request/response serialization
//! and the error type for API responses matching FastAPI's `{"detail": "..."}` format.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

// --- Request DTOs ---

/// Request body for arithmetic endpoints (`/add`, `/subtract`, `/multiply`, `/divide`).
#[derive(Debug, Deserialize)]
pub struct Operands {
    pub a: f64,
    pub b: f64,
}

// --- Response DTOs ---

/// Response body for arithmetic endpoints.
#[derive(Debug, Serialize)]
pub struct ResultResponse {
    pub result: f64,
}

/// Response body for the health check endpoint.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

// --- Error Types ---

/// JSON body for error responses, matching FastAPI's `{"detail": "..."}` format.
#[derive(Debug, Serialize)]
pub struct ErrorDetail {
    pub detail: String,
}

/// API error type that converts into an Axum HTTP response.
///
/// Produces JSON responses in the format `{"detail": "<message>"}` with the
/// specified HTTP status code, matching FastAPI's HTTPException behavior.
#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub message: String,
}

impl ApiError {
    /// Create a new API error with the given status code and message.
    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
        }
    }

    /// Create a 400 Bad Request error.
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, message)
    }

    /// Create a 404 Not Found error.
    #[allow(dead_code)]
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, message)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = ErrorDetail {
            detail: self.message,
        };
        (self.status, axum::Json(body)).into_response()
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.status, self.message)
    }
}

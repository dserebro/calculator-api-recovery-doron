/// Request/response models and error types for the Calculator API.
use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use std::fmt;

// --- Request Models ---

/// Request body for arithmetic calculation endpoints (add, subtract, multiply, divide).
#[derive(Debug, Deserialize)]
pub struct CalculationRequest {
    pub a: f64,
    pub b: f64,
}

// --- Response Models ---

/// Response body for arithmetic calculation endpoints.
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

/// Error detail response matching FastAPI's HTTPException format: {"detail": "message"}.
#[derive(Debug, Serialize)]
pub struct ErrorDetail {
    pub detail: String,
}

/// Application error type that maps to HTTP responses with FastAPI-compatible error format.
#[derive(Debug)]
pub enum ApiError {
    /// HTTP 400 Bad Request
    BadRequest(String),
    /// HTTP 404 Not Found
    NotFound(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::BadRequest(msg) => write!(f, "{}", msg),
            ApiError::NotFound(msg) => write!(f, "{}", msg),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let detail = ErrorDetail {
            detail: self.to_string(),
        };
        match self {
            ApiError::BadRequest(_) => HttpResponse::BadRequest().json(detail),
            ApiError::NotFound(_) => HttpResponse::NotFound().json(detail),
        }
    }
}

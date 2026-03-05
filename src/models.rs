/// Request and response models for the calculator API.
///
/// Serde-annotated structs matching the Python FastAPI/Pydantic models.
use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use std::fmt;

// --- Request Models ---

/// Request body for arithmetic endpoints (POST /add, /subtract, /multiply, /divide).
#[derive(Debug, Deserialize)]
pub struct CalculationRequest {
    pub a: f64,
    pub b: f64,
}

// --- Response Models ---

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

/// Application error type that renders to FastAPI-compatible JSON error responses.
///
/// Produces `{"detail": "..."}` matching Python's HTTPException format.
#[derive(Debug)]
pub enum AppError {
    /// Bad request (HTTP 400) - e.g., division by zero, unknown operation.
    BadRequest(String),
    /// Not found (HTTP 404) - e.g., calculation not found.
    NotFound(String),
}

/// Error response body matching FastAPI's HTTPException JSON format.
#[derive(Debug, Serialize)]
struct ErrorResponse {
    detail: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::BadRequest(msg) => write!(f, "{}", msg),
            AppError::NotFound(msg) => write!(f, "{}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let detail = self.to_string();
        let body = ErrorResponse { detail };
        match self {
            AppError::BadRequest(_) => HttpResponse::BadRequest().json(body),
            AppError::NotFound(_) => HttpResponse::NotFound().json(body),
        }
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AppError::BadRequest(_) => actix_web::http::StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
        }
    }
}

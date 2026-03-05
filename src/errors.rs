use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

/// Uniform error body matching FastAPI's `{"detail": "..."}` format.
#[derive(Debug, Serialize)]
pub struct ErrorBody {
    pub detail: String,
}

/// Application-level errors mapped to HTTP status codes.
#[derive(Debug)]
#[allow(dead_code)]
pub enum AppError {
    BadRequest(String),
    NotFound(String),
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
        let body = ErrorBody {
            detail: self.to_string(),
        };
        match self {
            AppError::BadRequest(_) => HttpResponse::BadRequest().json(body),
            AppError::NotFound(_) => HttpResponse::NotFound().json(body),
        }
    }
}

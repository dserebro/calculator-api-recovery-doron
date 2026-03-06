//! Request/response DTOs and database model structs.
//!
//! Matches the JSON shapes used by the Python FastAPI implementation.

use serde::{Deserialize, Serialize};

/// Request body for arithmetic endpoints (`/add`, `/subtract`, `/multiply`, `/divide`).
///
/// Equivalent to Python's `CalculationRequest(a: float, b: float)`.
#[derive(Debug, Deserialize)]
pub struct Operands {
    pub a: f64,
    pub b: f64,
}

/// Response body for arithmetic endpoints.
///
/// Equivalent to Python's `ResultResponse(result: float)`.
#[derive(Debug, Serialize)]
pub struct CalculationResult {
    pub result: f64,
}

/// Response body for the health check endpoint.
///
/// Equivalent to Python's `HealthResponse(status: str, version: str)`.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

/// Error response matching FastAPI's `{"detail": "..."}` format.
///
/// Used for all error responses to maintain parity with the Python API.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub detail: String,
}

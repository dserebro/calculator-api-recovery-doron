//! Request/response DTOs and error types.
//!
//! Defines the data structures used for HTTP request/response serialization
//! and the error type for API responses matching FastAPI's `{"detail": "..."}` format.

use axum::extract::{FromRequest, Request};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::de::DeserializeOwned;
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

// --- Validation Error Types (FastAPI-compatible) ---

/// A single validation error detail, matching FastAPI/Pydantic's format.
#[derive(Debug, Serialize)]
pub struct ValidationErrorItem {
    #[serde(rename = "type")]
    pub error_type: String,
    pub loc: Vec<serde_json::Value>,
    pub msg: String,
    pub input: serde_json::Value,
}

/// Validation error response matching FastAPI's 422 format.
#[derive(Debug, Serialize)]
pub struct ValidationErrorResponse {
    pub detail: Vec<ValidationErrorItem>,
}

// --- Custom JSON Extractor ---

/// Custom JSON extractor that returns FastAPI-compatible validation errors.
///
/// Unlike Axum's built-in `Json<T>`, this extractor returns JSON error responses
/// (not plain text) for deserialization failures, with status 422 and a body
/// matching FastAPI's validation error format.
pub struct ValidatedJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the raw body bytes
        let bytes = axum::body::Bytes::from_request(req, state)
            .await
            .map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    axum::Json(ErrorDetail {
                        detail: "Failed to read request body".to_string(),
                    }),
                )
                    .into_response()
            })?;

        // Parse as serde_json::Value first to preserve original input
        let raw_value: serde_json::Value = serde_json::from_slice(&bytes).map_err(|_| {
            (
                StatusCode::UNPROCESSABLE_ENTITY,
                axum::Json(ValidationErrorResponse {
                    detail: vec![ValidationErrorItem {
                        error_type: "json_invalid".to_string(),
                        loc: vec![serde_json::Value::String("body".to_string())],
                        msg: "Invalid JSON".to_string(),
                        input: serde_json::Value::Null,
                    }],
                }),
            )
                .into_response()
        })?;

        // Try to deserialize into the target type using serde_path_to_error
        match serde_path_to_error::deserialize::<_, T>(raw_value.clone()) {
            Ok(value) => Ok(ValidatedJson(value)),
            Err(err) => {
                let path_str = err.path().to_string();
                let inner_msg = err.inner().to_string();

                // Determine error type, message, field name, and input based on the serde error
                let (error_type, msg, field_name, input) =
                    if inner_msg.contains("missing field") {
                        // Extract field name from error message: "missing field `fieldname`"
                        let field = inner_msg
                            .split("missing field `")
                            .nth(1)
                            .and_then(|s| s.split('`').next())
                            .unwrap_or("")
                            .to_string();
                        (
                            "missing".to_string(),
                            "Field required".to_string(),
                            field,
                            raw_value.clone(),
                        )
                    } else if inner_msg.contains("invalid type") {
                        // Use the path from serde_path_to_error for the field name
                        let field = if !path_str.is_empty() && path_str != "." {
                            path_str.clone()
                        } else {
                            String::new()
                        };
                        // Extract the invalid value from the original input
                        let input_value = if !field.is_empty() {
                            raw_value
                                .get(&field)
                                .cloned()
                                .unwrap_or(serde_json::Value::Null)
                        } else {
                            raw_value.clone()
                        };
                        (
                            "float_parsing".to_string(),
                            "Input should be a valid number, unable to parse string as a number"
                                .to_string(),
                            field,
                            input_value,
                        )
                    } else {
                        let field = if !path_str.is_empty() && path_str != "." {
                            path_str.clone()
                        } else {
                            String::new()
                        };
                        (
                            "value_error".to_string(),
                            inner_msg,
                            field,
                            raw_value.clone(),
                        )
                    };

                // Build the "loc" array
                let mut loc: Vec<serde_json::Value> =
                    vec![serde_json::Value::String("body".to_string())];
                if !field_name.is_empty() {
                    loc.push(serde_json::Value::String(field_name));
                }

                let error_response = ValidationErrorResponse {
                    detail: vec![ValidationErrorItem {
                        error_type,
                        loc,
                        msg,
                        input,
                    }],
                };

                Err((StatusCode::UNPROCESSABLE_ENTITY, axum::Json(error_response)).into_response())
            }
        }
    }
}

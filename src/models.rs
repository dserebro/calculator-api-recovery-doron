//! Request/response DTOs and database model structs.

use axum::{
    body::Bytes,
    extract::FromRequest,
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

// --- Pydantic-compatible JSON Validation ---

/// Trait for types that declare their required JSON fields.
pub trait RequiredFields {
    fn required_field_names() -> &'static [&'static str];
}

impl RequiredFields for CalculationRequest {
    fn required_field_names() -> &'static [&'static str] {
        &["a", "b"]
    }
}

/// Custom JSON extractor that returns FastAPI/Pydantic-compatible 422 errors for missing fields.
pub struct ValidatedJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    T: serde::de::DeserializeOwned + RequiredFields + Send,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(
        req: axum::extract::Request,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let bytes = Bytes::from_request(req, state)
            .await
            .map_err(|e| {
                (StatusCode::UNPROCESSABLE_ENTITY, Json(serde_json::json!({
                    "detail": [{
                        "type": "json_invalid",
                        "loc": ["body"],
                        "msg": format!("JSON decode error: {}", e),
                        "input": null
                    }]
                }))).into_response()
            })?;

        // Parse as JSON Value first
        let value: serde_json::Value = serde_json::from_slice(&bytes)
            .map_err(|e| {
                (StatusCode::UNPROCESSABLE_ENTITY, Json(serde_json::json!({
                    "detail": [{
                        "type": "json_invalid",
                        "loc": ["body"],
                        "msg": format!("JSON decode error: {}", e),
                        "input": null
                    }]
                }))).into_response()
            })?;

        // Check required fields
        if let Some(obj) = value.as_object() {
            let required = T::required_field_names();
            let mut errors = Vec::new();
            for field in required {
                if !obj.contains_key(*field) {
                    errors.push(serde_json::json!({
                        "type": "missing",
                        "loc": ["body", *field],
                        "msg": "Field required",
                        "input": value
                    }));
                }
            }
            if !errors.is_empty() {
                return Err((StatusCode::UNPROCESSABLE_ENTITY, Json(serde_json::json!({
                    "detail": errors
                }))).into_response());
            }
        }

        // Deserialize as T
        let result: T = serde_json::from_value(value.clone())
            .map_err(|e| {
                (StatusCode::UNPROCESSABLE_ENTITY, Json(serde_json::json!({
                    "detail": [{
                        "type": "value_error",
                        "loc": ["body"],
                        "msg": e.to_string(),
                        "input": value
                    }]
                }))).into_response()
            })?;

        Ok(ValidatedJson(result))
    }
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

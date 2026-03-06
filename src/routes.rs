//! HTTP route handlers and router construction.
//!
//! Implements all HTTP endpoints matching the Python FastAPI server.

use axum::{
    extract::{FromRequest, Request},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum::body::Bytes;

use crate::core;
use crate::db::DbPool;
use crate::models::{CalculationResult, ErrorResponse, HealthResponse, Operands};

/// Custom JSON extractor that returns FastAPI-compatible validation error responses.
///
/// Axum's default `Json<T>` extractor returns plain text on rejection.
/// FastAPI returns JSON `{"detail": [...]}` with Pydantic V2 error objects for validation errors.
/// This extractor buffers the request body, attempts deserialization, and on failure
/// constructs a response matching the FastAPI/Pydantic V2 error format.
struct AppJson<T>(T);

impl<S, T> FromRequest<S> for AppJson<T>
where
    T: serde::de::DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = (StatusCode, axum::Json<serde_json::Value>);

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // Check content-type header
        let has_json_ct = req
            .headers()
            .get(axum::http::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(|v| v.starts_with("application/json"))
            .unwrap_or(false);

        if !has_json_ct {
            return Err((
                StatusCode::UNPROCESSABLE_ENTITY,
                axum::Json(serde_json::json!({
                    "detail": [{"msg": "Missing content-type header", "type": "value_error"}]
                })),
            ));
        }

        // Buffer the body bytes
        let bytes = Bytes::from_request(req, state).await.map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                axum::Json(serde_json::json!({
                    "detail": [{"msg": "Failed to read request body", "type": "value_error"}]
                })),
            )
        })?;

        // Try to deserialize using serde_path_to_error for field path info
        let deserializer = &mut serde_json::Deserializer::from_slice(&bytes);
        match serde_path_to_error::deserialize::<_, T>(deserializer) {
            Ok(value) => Ok(AppJson(value)),
            Err(path_err) => {
                // Parse the original body as a generic JSON value for the "input" field
                let input_value = serde_json::from_slice::<serde_json::Value>(&bytes)
                    .unwrap_or(serde_json::Value::Null);

                let field_path = path_err.path().to_string();
                let inner_msg = path_err.inner().to_string();
                let detail = build_pydantic_error(&field_path, &inner_msg, &input_value);

                Err((
                    StatusCode::UNPROCESSABLE_ENTITY,
                    axum::Json(serde_json::json!({ "detail": detail })),
                ))
            }
        }
    }
}

/// Build a Pydantic V2-compatible error array from a serde deserialization error.
///
/// `field_path` is from `serde_path_to_error` (e.g., "." for root, "a" for field `a`).
/// `inner_msg` is the serde_json error message (e.g., "missing field `b`...").
///
/// Matches the FastAPI validation error format:
/// - Missing field: `{"type": "missing", "loc": ["body", "<field>"], "msg": "Field required", "input": <body>}`
/// - Invalid type (float): `{"type": "float_parsing", "loc": ["body", "<field>"], "msg": "Input should be a valid number, unable to parse string as a number", "input": "<value>"}`
fn build_pydantic_error(
    field_path: &str,
    inner_msg: &str,
    input_value: &serde_json::Value,
) -> Vec<serde_json::Value> {
    // Missing field: inner_msg contains "missing field `<name>`"
    if let Some(field) = extract_between(inner_msg, "missing field `", "`") {
        return vec![serde_json::json!({
            "type": "missing",
            "loc": ["body", field],
            "msg": "Field required",
            "input": input_value
        })];
    }

    // Invalid type: inner_msg contains "invalid type:"
    if inner_msg.contains("invalid type:") {
        // field_path from serde_path_to_error gives us the field name (e.g., "a")
        let field = if field_path == "." { "" } else { field_path };
        // Extract the bad value from the input object
        let bad_input = input_value
            .get(field)
            .cloned()
            .unwrap_or(serde_json::Value::Null);
        return vec![serde_json::json!({
            "type": "float_parsing",
            "loc": ["body", field],
            "msg": "Input should be a valid number, unable to parse string as a number",
            "input": bad_input
        })];
    }

    // Fallback for unrecognized errors
    vec![serde_json::json!({
        "type": "value_error",
        "loc": ["body"],
        "msg": inner_msg
    })]
}

/// Extract the substring between `start` and `end` delimiters.
fn extract_between<'a>(s: &'a str, start: &str, end: &str) -> Option<&'a str> {
    let start_idx = s.find(start)? + start.len();
    let rest = &s[start_idx..];
    let end_idx = rest.find(end)?;
    Some(&rest[..end_idx])
}

/// Build the Axum router with all endpoint routes.
///
/// The router is configured with the database pool as shared state,
/// which is used by CRUD endpoints (added in Milestone 2).
pub fn create_router(pool: DbPool) -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .route("/add", post(add_handler))
        .route("/subtract", post(subtract_handler))
        .route("/multiply", post(multiply_handler))
        .route("/divide", post(divide_handler))
        .with_state(pool)
}

/// GET /health — Returns service health status.
///
/// Response: `{"status": "ok", "version": "0.1.0"}` with HTTP 200.
async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: "0.1.0".to_string(),
    })
}

/// POST /add — Add two numbers.
///
/// Request: `{"a": f64, "b": f64}`
/// Response: `{"result": f64}` with HTTP 200.
async fn add_handler(AppJson(payload): AppJson<Operands>) -> Json<CalculationResult> {
    Json(CalculationResult {
        result: core::add(payload.a, payload.b),
    })
}

/// POST /subtract — Subtract b from a.
///
/// Request: `{"a": f64, "b": f64}`
/// Response: `{"result": f64}` with HTTP 200.
async fn subtract_handler(AppJson(payload): AppJson<Operands>) -> Json<CalculationResult> {
    Json(CalculationResult {
        result: core::subtract(payload.a, payload.b),
    })
}

/// POST /multiply — Multiply two numbers.
///
/// Request: `{"a": f64, "b": f64}`
/// Response: `{"result": f64}` with HTTP 200.
async fn multiply_handler(AppJson(payload): AppJson<Operands>) -> Json<CalculationResult> {
    Json(CalculationResult {
        result: core::multiply(payload.a, payload.b),
    })
}

/// POST /divide — Divide a by b.
///
/// Request: `{"a": f64, "b": f64}`
/// Response: `{"result": f64}` with HTTP 200.
/// Error: HTTP 400 with `{"detail": "Cannot divide by zero"}` when b is 0.
async fn divide_handler(
    AppJson(payload): AppJson<Operands>,
) -> Result<Json<CalculationResult>, impl IntoResponse> {
    match core::divide(payload.a, payload.b) {
        Ok(result) => Ok(Json(CalculationResult { result })),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                detail: e.to_string(),
            }),
        )),
    }
}

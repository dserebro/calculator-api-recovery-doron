/// HTTP route definitions and handlers for the Calculator API.
use actix_web::{web, HttpResponse, http::StatusCode};

use crate::core;
use crate::models::{ApiError, CalculationRequest, HealthResponse, ResultResponse, ValidationErrorDetail, ValidationErrorItem};

/// Build a FastAPI/Pydantic-compatible validation error response (422) from a serde_json error.
fn build_validation_error(e: &serde_json::Error, body: &str) -> HttpResponse {
    let err_msg = format!("{}", e);
    let input_value: serde_json::Value = serde_json::from_str(body).unwrap_or(serde_json::Value::Null);

    // Determine error type and extract field name from serde error message
    let (error_type, field_name, msg, input) = if let Some(rest) = err_msg.strip_prefix("missing field `") {
        // e.g. "missing field `b` at line 1 column 10"
        let field = rest.split('`').next().unwrap_or("unknown");
        ("missing".to_string(), field.to_string(), "Field required".to_string(), input_value)
    } else if err_msg.contains("invalid type:") {
        // e.g. "invalid type: string \"not_a_number\", expected f64 at line 1 column 20"
        // Extract field name from the input by checking which field has wrong type
        let field = guess_invalid_field(&input_value);
        let field_input = input_value.get(&field).cloned().unwrap_or(serde_json::Value::Null);
        (
            "float_parsing".to_string(),
            field,
            "Input should be a valid number, unable to parse string as a number".to_string(),
            field_input,
        )
    } else {
        ("value_error".to_string(), "unknown".to_string(), err_msg, input_value)
    };

    let item = ValidationErrorItem {
        error_type,
        loc: vec![
            serde_json::Value::String("body".to_string()),
            serde_json::Value::String(field_name),
        ],
        msg,
        input,
    };

    HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY)
        .json(ValidationErrorDetail { detail: vec![item] })
}

/// Try to guess which field has an invalid type by checking non-numeric string values.
fn guess_invalid_field(input: &serde_json::Value) -> String {
    if let Some(obj) = input.as_object() {
        for (key, value) in obj {
            if value.is_string() {
                return key.clone();
            }
        }
    }
    "unknown".to_string()
}

/// Helper to parse JSON body, returning 422 on deserialization errors (matching FastAPI behavior).
fn parse_json(body: &str) -> Result<CalculationRequest, HttpResponse> {
    serde_json::from_str(body).map_err(|e| build_validation_error(&e, body))
}

/// Health check handler.
///
/// Returns the service status and version.
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok".to_string(),
        version: "0.1.0".to_string(),
    })
}

/// Add two numbers.
pub async fn add(body: String) -> HttpResponse {
    let req = match parse_json(&body) {
        Ok(r) => r,
        Err(resp) => return resp,
    };
    let result = core::add(req.a, req.b);
    HttpResponse::Ok().json(ResultResponse { result })
}

/// Subtract b from a.
pub async fn subtract(body: String) -> HttpResponse {
    let req = match parse_json(&body) {
        Ok(r) => r,
        Err(resp) => return resp,
    };
    let result = core::subtract(req.a, req.b);
    HttpResponse::Ok().json(ResultResponse { result })
}

/// Multiply two numbers.
pub async fn multiply(body: String) -> HttpResponse {
    let req = match parse_json(&body) {
        Ok(r) => r,
        Err(resp) => return resp,
    };
    let result = core::multiply(req.a, req.b);
    HttpResponse::Ok().json(ResultResponse { result })
}

/// Divide a by b.
///
/// Returns HTTP 400 if b is zero.
pub async fn divide(body: String) -> Result<HttpResponse, ApiError> {
    let req = match parse_json(&body) {
        Ok(r) => r,
        Err(resp) => return Ok(resp),
    };
    match core::divide(req.a, req.b) {
        Ok(result) => Ok(HttpResponse::Ok().json(ResultResponse { result })),
        Err(msg) => Err(ApiError::BadRequest(msg)),
    }
}

/// Configure all routes for the application.
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/health").route(web::get().to(health_check)),
    )
    .service(
        web::resource("/add").route(web::post().to(add)),
    )
    .service(
        web::resource("/subtract").route(web::post().to(subtract)),
    )
    .service(
        web::resource("/multiply").route(web::post().to(multiply)),
    )
    .service(
        web::resource("/divide").route(web::post().to(divide)),
    );
}

/// HTTP route definitions and handlers for the Calculator API.
use actix_web::{web, HttpResponse, http::StatusCode};

use crate::core;
use crate::models::{ApiError, CalculationRequest, HealthResponse, ResultResponse, ValidationErrorDetail, ValidationErrorItem};

/// Required fields for CalculationRequest.
const REQUIRED_FIELDS: &[&str] = &["a", "b"];

/// Build a FastAPI/Pydantic-compatible validation error response (422) from a serde_json error.
fn build_validation_error(e: &serde_json::Error, body: &str) -> HttpResponse {
    let err_msg = format!("{}", e);
    let input_value: serde_json::Value = serde_json::from_str(body).unwrap_or(serde_json::Value::Null);

    let items = if err_msg.contains("missing field") {
        // Check ALL required fields, not just the first one serde reports
        let mut missing_items = Vec::new();
        if let Some(obj) = input_value.as_object() {
            for &field in REQUIRED_FIELDS {
                if !obj.contains_key(field) {
                    missing_items.push(ValidationErrorItem {
                        error_type: "missing".to_string(),
                        loc: vec![
                            serde_json::Value::String("body".to_string()),
                            serde_json::Value::String(field.to_string()),
                        ],
                        msg: "Field required".to_string(),
                        input: input_value.clone(),
                    });
                }
            }
        }
        if missing_items.is_empty() {
            // Fallback: extract field from error message
            let field = err_msg.strip_prefix("missing field `")
                .and_then(|r| r.split('`').next())
                .unwrap_or("unknown");
            vec![ValidationErrorItem {
                error_type: "missing".to_string(),
                loc: vec![
                    serde_json::Value::String("body".to_string()),
                    serde_json::Value::String(field.to_string()),
                ],
                msg: "Field required".to_string(),
                input: input_value,
            }]
        } else {
            missing_items
        }
    } else if err_msg.contains("invalid type:") {
        let field = guess_invalid_field(&input_value);
        let field_input = input_value.get(&field).cloned().unwrap_or(serde_json::Value::Null);
        vec![ValidationErrorItem {
            error_type: "float_parsing".to_string(),
            loc: vec![
                serde_json::Value::String("body".to_string()),
                serde_json::Value::String(field),
            ],
            msg: "Input should be a valid number, unable to parse string as a number".to_string(),
            input: field_input,
        }]
    } else {
        vec![ValidationErrorItem {
            error_type: "value_error".to_string(),
            loc: vec![
                serde_json::Value::String("body".to_string()),
                serde_json::Value::String("unknown".to_string()),
            ],
            msg: err_msg,
            input: input_value,
        }]
    };

    HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY)
        .json(ValidationErrorDetail { detail: items })
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

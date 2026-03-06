//! Integration tests for health check and arithmetic endpoints.
//!
//! Uses in-memory SQLite and Axum's test infrastructure to verify
//! endpoint behavior matches the Python FastAPI implementation.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

use calculator_api::{db, routes};

/// Helper: create an in-memory SQLite pool and build the Axum router.
async fn app() -> axum::Router {
    let pool = db::init_pool("sqlite::memory:")
        .await
        .expect("Failed to create in-memory DB pool");
    routes::create_router(pool)
}

/// Helper: send a request and return (status, json body).
async fn send_request(
    app: axum::Router,
    method: &str,
    uri: &str,
    body: Option<Value>,
) -> (StatusCode, Value) {
    let body = match body {
        Some(v) => Body::from(serde_json::to_string(&v).unwrap()),
        None => Body::empty(),
    };

    let mut builder = Request::builder().uri(uri).method(method);
    if method == "POST" {
        builder = builder.header("content-type", "application/json");
    }
    let request = builder.body(body).unwrap();

    let response = app.oneshot(request).await.unwrap();
    let status = response.status();
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = if body_bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&body_bytes).unwrap_or(Value::Null)
    };
    (status, json)
}

// ========================================================================
// Health Check Tests
// ========================================================================

#[tokio::test]
async fn test_health_check() {
    let app = app().await;
    let (status, json) = send_request(app, "GET", "/health", None).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["status"], "ok");
    assert_eq!(json["version"], "0.1.0");
}

// ========================================================================
// POST /add Tests
// ========================================================================

#[tokio::test]
async fn test_add_positive_numbers() {
    let app = app().await;
    let (status, json) = send_request(
        app,
        "POST",
        "/add",
        Some(json!({"a": 2.0, "b": 3.0})),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["result"], 5.0);
}

#[tokio::test]
async fn test_add_negative_numbers() {
    let app = app().await;
    let (status, json) = send_request(
        app,
        "POST",
        "/add",
        Some(json!({"a": -1.0, "b": -2.0})),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["result"], -3.0);
}

#[tokio::test]
async fn test_add_zero() {
    let app = app().await;
    let (status, json) = send_request(
        app,
        "POST",
        "/add",
        Some(json!({"a": 0.0, "b": 0.0})),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["result"], 0.0);
}

// ========================================================================
// POST /subtract Tests
// ========================================================================

#[tokio::test]
async fn test_subtract_positive() {
    let app = app().await;
    let (status, json) = send_request(
        app,
        "POST",
        "/subtract",
        Some(json!({"a": 5.0, "b": 3.0})),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["result"], 2.0);
}

#[tokio::test]
async fn test_subtract_negative_result() {
    let app = app().await;
    let (status, json) = send_request(
        app,
        "POST",
        "/subtract",
        Some(json!({"a": 3.0, "b": 5.0})),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["result"], -2.0);
}

// ========================================================================
// POST /multiply Tests
// ========================================================================

#[tokio::test]
async fn test_multiply_positive() {
    let app = app().await;
    let (status, json) = send_request(
        app,
        "POST",
        "/multiply",
        Some(json!({"a": 2.0, "b": 3.0})),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["result"], 6.0);
}

#[tokio::test]
async fn test_multiply_by_zero() {
    let app = app().await;
    let (status, json) = send_request(
        app,
        "POST",
        "/multiply",
        Some(json!({"a": 5.0, "b": 0.0})),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["result"], 0.0);
}

#[tokio::test]
async fn test_multiply_negative() {
    let app = app().await;
    let (status, json) = send_request(
        app,
        "POST",
        "/multiply",
        Some(json!({"a": -2.0, "b": 3.0})),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["result"], -6.0);
}

// ========================================================================
// POST /divide Tests
// ========================================================================

#[tokio::test]
async fn test_divide_positive() {
    let app = app().await;
    let (status, json) = send_request(
        app,
        "POST",
        "/divide",
        Some(json!({"a": 6.0, "b": 3.0})),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["result"], 2.0);
}

#[tokio::test]
async fn test_divide_fractional() {
    let app = app().await;
    let (status, json) = send_request(
        app,
        "POST",
        "/divide",
        Some(json!({"a": 1.0, "b": 3.0})),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    // Check approximate equality for floating point
    let result = json["result"].as_f64().unwrap();
    assert!((result - 1.0 / 3.0).abs() < f64::EPSILON);
}

#[tokio::test]
async fn test_divide_by_zero_returns_400() {
    let app = app().await;
    let (status, json) = send_request(
        app,
        "POST",
        "/divide",
        Some(json!({"a": 5.0, "b": 0.0})),
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(json["detail"], "Cannot divide by zero");
}

#[tokio::test]
async fn test_divide_zero_by_nonzero() {
    let app = app().await;
    let (status, json) = send_request(
        app,
        "POST",
        "/divide",
        Some(json!({"a": 0.0, "b": 5.0})),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["result"], 0.0);
}

// ========================================================================
// Error Handling Tests
// ========================================================================

#[tokio::test]
async fn test_add_missing_fields_returns_error() {
    let app = app().await;
    let (status, _json) = send_request(
        app,
        "POST",
        "/add",
        Some(json!({"a": 5.0})), // missing "b"
    )
    .await;

    // Axum/serde returns 422 Unprocessable Entity for missing fields
    assert!(
        status == StatusCode::UNPROCESSABLE_ENTITY || status == StatusCode::BAD_REQUEST,
        "Expected 422 or 400, got {}",
        status
    );
}

#[tokio::test]
async fn test_add_invalid_json_returns_error() {
    let app = app().await;

    let request = Request::builder()
        .uri("/add")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from("not valid json"))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert!(
        response.status() == StatusCode::UNPROCESSABLE_ENTITY
            || response.status() == StatusCode::BAD_REQUEST,
        "Expected 422 or 400, got {}",
        response.status()
    );
}

#[tokio::test]
async fn test_add_with_integers() {
    // Python's Pydantic accepts integers and coerces them to floats
    // serde should also accept integer JSON values for f64 fields
    let app = app().await;
    let (status, json) = send_request(
        app,
        "POST",
        "/add",
        Some(json!({"a": 2, "b": 3})),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["result"], 5.0);
}

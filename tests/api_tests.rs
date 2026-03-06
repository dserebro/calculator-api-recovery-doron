//! Integration tests for health check and arithmetic endpoints.
//!
//! Tests use an in-memory SQLite database and Axum's tower::ServiceExt
//! for in-process HTTP testing without starting a real server.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

use calculator_api::db;
use calculator_api::routes;

/// Helper: create a test app with in-memory SQLite.
async fn test_app() -> axum::Router {
    let pool = db::init_pool("sqlite::memory:")
        .await
        .expect("Failed to create in-memory DB pool");
    routes::create_router(pool)
}

/// Helper: extract JSON body as serde_json::Value from a response.
async fn body_json(body: Body) -> serde_json::Value {
    let bytes = body.collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

// =============================================================================
// Health Check
// =============================================================================

#[tokio::test]
async fn test_health_check() {
    let app = test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let json = body_json(response.into_body()).await;
    assert_eq!(json["status"], "ok");
    assert_eq!(json["version"], "0.1.0");
}

// =============================================================================
// POST /add
// =============================================================================

#[tokio::test]
async fn test_add_positive_numbers() {
    let app = test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/add")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"a": 2, "b": 3}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let json = body_json(response.into_body()).await;
    assert_eq!(json["result"], 5.0);
}

#[tokio::test]
async fn test_add_negative_numbers() {
    let app = test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/add")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"a": -2, "b": -3}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let json = body_json(response.into_body()).await;
    assert_eq!(json["result"], -5.0);
}

#[tokio::test]
async fn test_add_with_zero() {
    let app = test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/add")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"a": 0, "b": 5}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let json = body_json(response.into_body()).await;
    assert_eq!(json["result"], 5.0);
}

// =============================================================================
// POST /subtract
// =============================================================================

#[tokio::test]
async fn test_subtract_positive_numbers() {
    let app = test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/subtract")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"a": 10, "b": 3}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let json = body_json(response.into_body()).await;
    assert_eq!(json["result"], 7.0);
}

#[tokio::test]
async fn test_subtract_negative_result() {
    let app = test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/subtract")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"a": 3, "b": 10}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let json = body_json(response.into_body()).await;
    assert_eq!(json["result"], -7.0);
}

// =============================================================================
// POST /multiply
// =============================================================================

#[tokio::test]
async fn test_multiply_positive_numbers() {
    let app = test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/multiply")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"a": 4, "b": 5}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let json = body_json(response.into_body()).await;
    assert_eq!(json["result"], 20.0);
}

#[tokio::test]
async fn test_multiply_with_zero() {
    let app = test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/multiply")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"a": 100, "b": 0}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let json = body_json(response.into_body()).await;
    assert_eq!(json["result"], 0.0);
}

#[tokio::test]
async fn test_multiply_negative_numbers() {
    let app = test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/multiply")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"a": -3, "b": -4}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let json = body_json(response.into_body()).await;
    assert_eq!(json["result"], 12.0);
}

// =============================================================================
// POST /divide
// =============================================================================

#[tokio::test]
async fn test_divide_positive_numbers() {
    let app = test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/divide")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"a": 10, "b": 2}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let json = body_json(response.into_body()).await;
    assert_eq!(json["result"], 5.0);
}

#[tokio::test]
async fn test_divide_fractional_result() {
    let app = test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/divide")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"a": 7, "b": 2}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let json = body_json(response.into_body()).await;
    assert_eq!(json["result"], 3.5);
}

#[tokio::test]
async fn test_divide_by_zero_returns_400() {
    let app = test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/divide")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"a": 10, "b": 0}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let json = body_json(response.into_body()).await;
    assert_eq!(json["detail"], "Cannot divide by zero");
}

#[tokio::test]
async fn test_divide_zero_by_zero_returns_400() {
    let app = test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/divide")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"a": 0, "b": 0}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let json = body_json(response.into_body()).await;
    assert_eq!(json["detail"], "Cannot divide by zero");
}

// =============================================================================
// Error cases - Invalid payloads
// =============================================================================

#[tokio::test]
async fn test_add_missing_fields_returns_422() {
    let app = test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/add")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"a": 5}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    // Axum returns 422 Unprocessable Entity for deserialization errors
    // (FastAPI also returns 422 for validation errors)
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_add_invalid_json_returns_400() {
    let app = test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/add")
                .header("content-type", "application/json")
                .body(Body::from(r#"not json"#))
                .unwrap(),
        )
        .await
        .unwrap();

    // Axum returns 400 for invalid JSON syntax
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

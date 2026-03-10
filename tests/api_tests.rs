//! Integration tests for the calculator API endpoints.
//!
//! These tests spin up the Axum app in-process with an in-memory SQLite database
//! and verify all endpoint behaviors match the Python FastAPI implementation.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

/// Helper: create an in-memory SQLite pool and initialize the schema.
async fn setup_pool() -> sqlx::SqlitePool {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("Failed to create in-memory SQLite pool");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS calculations (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            operation   TEXT NOT NULL,
            a           REAL NOT NULL,
            b           REAL NOT NULL,
            result      REAL NOT NULL,
            created_at  TEXT NOT NULL DEFAULT (datetime('now'))
        )
        "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create calculations table");

    pool
}

/// Helper: create the app router with an in-memory database.
async fn app() -> axum::Router {
    let pool = setup_pool().await;
    calculator_api::routes::create_router(pool)
}

/// Helper: send a JSON POST request and return the response.
async fn post_json(
    app: axum::Router,
    uri: &str,
    body: serde_json::Value,
) -> (StatusCode, serde_json::Value) {
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(uri)
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    let status = response.status();
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    (status, json)
}

/// Helper: send a GET request and return the response.
async fn get_request(app: axum::Router, uri: &str) -> (StatusCode, serde_json::Value) {
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(uri)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let status = response.status();
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    (status, json)
}

// ============================================================
// Health Check Tests
// ============================================================

#[tokio::test]
async fn test_health_check() {
    let app = app().await;
    let (status, json) = get_request(app, "/health").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["status"], "ok");
    assert_eq!(json["version"], "0.1.0");
}

// ============================================================
// Addition Tests
// ============================================================

#[tokio::test]
async fn test_add_positive_numbers() {
    let app = app().await;
    let (status, json) = post_json(app, "/add", serde_json::json!({"a": 2, "b": 3})).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["result"], 5.0);
}

#[tokio::test]
async fn test_add_negative_numbers() {
    let app = app().await;
    let (status, json) = post_json(app, "/add", serde_json::json!({"a": -2, "b": -3})).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["result"], -5.0);
}

#[tokio::test]
async fn test_add_with_zero() {
    let app = app().await;
    let (status, json) = post_json(app, "/add", serde_json::json!({"a": 0, "b": 5})).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["result"], 5.0);
}

#[tokio::test]
async fn test_add_decimals() {
    let app = app().await;
    let (status, json) = post_json(app, "/add", serde_json::json!({"a": 1.5, "b": 2.5})).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["result"], 4.0);
}

// ============================================================
// Subtraction Tests
// ============================================================

#[tokio::test]
async fn test_subtract_positive_numbers() {
    let app = app().await;
    let (status, json) =
        post_json(app, "/subtract", serde_json::json!({"a": 10, "b": 3})).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["result"], 7.0);
}

#[tokio::test]
async fn test_subtract_negative_result() {
    let app = app().await;
    let (status, json) =
        post_json(app, "/subtract", serde_json::json!({"a": 3, "b": 10})).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["result"], -7.0);
}

#[tokio::test]
async fn test_subtract_with_zero() {
    let app = app().await;
    let (status, json) =
        post_json(app, "/subtract", serde_json::json!({"a": 5, "b": 0})).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["result"], 5.0);
}

// ============================================================
// Multiplication Tests
// ============================================================

#[tokio::test]
async fn test_multiply_positive_numbers() {
    let app = app().await;
    let (status, json) =
        post_json(app, "/multiply", serde_json::json!({"a": 4, "b": 5})).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["result"], 20.0);
}

#[tokio::test]
async fn test_multiply_with_zero() {
    let app = app().await;
    let (status, json) =
        post_json(app, "/multiply", serde_json::json!({"a": 0, "b": 5})).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["result"], 0.0);
}

#[tokio::test]
async fn test_multiply_negative_numbers() {
    let app = app().await;
    let (status, json) =
        post_json(app, "/multiply", serde_json::json!({"a": -4, "b": -5})).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["result"], 20.0);
}

#[tokio::test]
async fn test_multiply_mixed_signs() {
    let app = app().await;
    let (status, json) =
        post_json(app, "/multiply", serde_json::json!({"a": -4, "b": 5})).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["result"], -20.0);
}

// ============================================================
// Division Tests
// ============================================================

#[tokio::test]
async fn test_divide_positive_numbers() {
    let app = app().await;
    let (status, json) =
        post_json(app, "/divide", serde_json::json!({"a": 10, "b": 2})).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["result"], 5.0);
}

#[tokio::test]
async fn test_divide_with_decimals() {
    let app = app().await;
    let (status, json) =
        post_json(app, "/divide", serde_json::json!({"a": 7, "b": 2})).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["result"], 3.5);
}

#[tokio::test]
async fn test_divide_by_zero_returns_400() {
    let app = app().await;
    let (status, json) =
        post_json(app, "/divide", serde_json::json!({"a": 1, "b": 0})).await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(json["detail"], "Cannot divide by zero");
}

#[tokio::test]
async fn test_divide_zero_by_nonzero() {
    let app = app().await;
    let (status, json) =
        post_json(app, "/divide", serde_json::json!({"a": 0, "b": 5})).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["result"], 0.0);
}

#[tokio::test]
async fn test_divide_negative_numbers() {
    let app = app().await;
    let (status, json) =
        post_json(app, "/divide", serde_json::json!({"a": -10, "b": -2})).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["result"], 5.0);
}

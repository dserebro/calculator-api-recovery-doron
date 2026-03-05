/// Integration tests for the Calculator API stateless endpoints.
///
/// Tests use Actix-web's test utilities to issue HTTP requests and
/// validate responses match the Python FastAPI implementation.
use actix_web::{test, App};
use serde_json::Value;

use calculator_api::routes;

/// Helper to create an Actix-web test app with all routes configured.
fn create_test_app() -> App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse<impl actix_web::body::MessageBody>,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new().configure(routes::configure)
}

// =============================================================================
// Health Check Tests
// =============================================================================

#[actix_web::test]
async fn test_health_check_returns_200() {
    let app = test::init_service(create_test_app()).await;
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
}

#[actix_web::test]
async fn test_health_check_returns_correct_body() {
    let app = test::init_service(create_test_app()).await;
    let req = test::TestRequest::get().uri("/health").to_request();
    let body: Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(body["status"], "ok");
    assert_eq!(body["version"], "0.1.0");
}

// =============================================================================
// Addition Tests
// =============================================================================

#[actix_web::test]
async fn test_add_positive_numbers() {
    let app = test::init_service(create_test_app()).await;
    let req = test::TestRequest::post()
        .uri("/add")
        .set_json(serde_json::json!({"a": 5.0, "b": 3.0}))
        .to_request();
    let body: Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(body["result"], 8.0);
}

#[actix_web::test]
async fn test_add_negative_numbers() {
    let app = test::init_service(create_test_app()).await;
    let req = test::TestRequest::post()
        .uri("/add")
        .set_json(serde_json::json!({"a": -5.0, "b": -3.0}))
        .to_request();
    let body: Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(body["result"], -8.0);
}

#[actix_web::test]
async fn test_add_with_zero() {
    let app = test::init_service(create_test_app()).await;
    let req = test::TestRequest::post()
        .uri("/add")
        .set_json(serde_json::json!({"a": 5.0, "b": 0.0}))
        .to_request();
    let body: Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(body["result"], 5.0);
}

#[actix_web::test]
async fn test_add_fractional_numbers() {
    let app = test::init_service(create_test_app()).await;
    let req = test::TestRequest::post()
        .uri("/add")
        .set_json(serde_json::json!({"a": 1.5, "b": 2.5}))
        .to_request();
    let body: Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(body["result"], 4.0);
}

// =============================================================================
// Subtraction Tests
// =============================================================================

#[actix_web::test]
async fn test_subtract_positive_numbers() {
    let app = test::init_service(create_test_app()).await;
    let req = test::TestRequest::post()
        .uri("/subtract")
        .set_json(serde_json::json!({"a": 10.0, "b": 4.0}))
        .to_request();
    let body: Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(body["result"], 6.0);
}

#[actix_web::test]
async fn test_subtract_negative_result() {
    let app = test::init_service(create_test_app()).await;
    let req = test::TestRequest::post()
        .uri("/subtract")
        .set_json(serde_json::json!({"a": 3.0, "b": 7.0}))
        .to_request();
    let body: Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(body["result"], -4.0);
}

#[actix_web::test]
async fn test_subtract_with_zero() {
    let app = test::init_service(create_test_app()).await;
    let req = test::TestRequest::post()
        .uri("/subtract")
        .set_json(serde_json::json!({"a": 5.0, "b": 0.0}))
        .to_request();
    let body: Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(body["result"], 5.0);
}

// =============================================================================
// Multiplication Tests
// =============================================================================

#[actix_web::test]
async fn test_multiply_positive_numbers() {
    let app = test::init_service(create_test_app()).await;
    let req = test::TestRequest::post()
        .uri("/multiply")
        .set_json(serde_json::json!({"a": 7.0, "b": 6.0}))
        .to_request();
    let body: Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(body["result"], 42.0);
}

#[actix_web::test]
async fn test_multiply_with_zero() {
    let app = test::init_service(create_test_app()).await;
    let req = test::TestRequest::post()
        .uri("/multiply")
        .set_json(serde_json::json!({"a": 5.0, "b": 0.0}))
        .to_request();
    let body: Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(body["result"], 0.0);
}

#[actix_web::test]
async fn test_multiply_negative_numbers() {
    let app = test::init_service(create_test_app()).await;
    let req = test::TestRequest::post()
        .uri("/multiply")
        .set_json(serde_json::json!({"a": -3.0, "b": -4.0}))
        .to_request();
    let body: Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(body["result"], 12.0);
}

// =============================================================================
// Division Tests
// =============================================================================

#[actix_web::test]
async fn test_divide_positive_numbers() {
    let app = test::init_service(create_test_app()).await;
    let req = test::TestRequest::post()
        .uri("/divide")
        .set_json(serde_json::json!({"a": 10.0, "b": 2.0}))
        .to_request();
    let body: Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(body["result"], 5.0);
}

#[actix_web::test]
async fn test_divide_fractional_result() {
    let app = test::init_service(create_test_app()).await;
    let req = test::TestRequest::post()
        .uri("/divide")
        .set_json(serde_json::json!({"a": 7.0, "b": 2.0}))
        .to_request();
    let body: Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(body["result"], 3.5);
}

#[actix_web::test]
async fn test_divide_by_zero_returns_400() {
    let app = test::init_service(create_test_app()).await;
    let req = test::TestRequest::post()
        .uri("/divide")
        .set_json(serde_json::json!({"a": 5.0, "b": 0.0}))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);
}

#[actix_web::test]
async fn test_divide_by_zero_returns_correct_error_body() {
    let app = test::init_service(create_test_app()).await;
    let req = test::TestRequest::post()
        .uri("/divide")
        .set_json(serde_json::json!({"a": 5.0, "b": 0.0}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;

    assert_eq!(body["detail"], "Cannot divide by zero");
}

#[actix_web::test]
async fn test_divide_zero_by_nonzero() {
    let app = test::init_service(create_test_app()).await;
    let req = test::TestRequest::post()
        .uri("/divide")
        .set_json(serde_json::json!({"a": 0.0, "b": 5.0}))
        .to_request();
    let body: Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(body["result"], 0.0);
}

// =============================================================================
// Response Format Tests
// =============================================================================

#[actix_web::test]
async fn test_add_returns_200_status() {
    let app = test::init_service(create_test_app()).await;
    let req = test::TestRequest::post()
        .uri("/add")
        .set_json(serde_json::json!({"a": 1.0, "b": 1.0}))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
}

#[actix_web::test]
async fn test_response_has_result_key() {
    let app = test::init_service(create_test_app()).await;
    let req = test::TestRequest::post()
        .uri("/add")
        .set_json(serde_json::json!({"a": 1.0, "b": 1.0}))
        .to_request();
    let body: Value = test::call_and_read_body_json(&app, req).await;

    assert!(body.get("result").is_some(), "Response should have 'result' key");
}

#[actix_web::test]
async fn test_health_response_has_status_and_version_keys() {
    let app = test::init_service(create_test_app()).await;
    let req = test::TestRequest::get().uri("/health").to_request();
    let body: Value = test::call_and_read_body_json(&app, req).await;

    assert!(body.get("status").is_some(), "Response should have 'status' key");
    assert!(body.get("version").is_some(), "Response should have 'version' key");
}

use actix_web::{get, post, web, HttpResponse};

use crate::core;
use crate::models::{
    validate_calculation_request, ErrorResponse, HealthResponse, ResultResponse,
    ValidationErrorResponse,
};

#[get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok".to_string(),
        version: "0.1.0".to_string(),
    })
}

fn parse_and_validate(body: web::Json<serde_json::Value>) -> Result<(f64, f64), HttpResponse> {
    match validate_calculation_request(&body) {
        Ok(pair) => Ok(pair),
        Err(errors) => Err(HttpResponse::UnprocessableEntity()
            .json(ValidationErrorResponse { detail: errors })),
    }
}

#[post("/add")]
async fn api_add(body: web::Json<serde_json::Value>) -> HttpResponse {
    let (a, b) = match parse_and_validate(body) {
        Ok(v) => v,
        Err(resp) => return resp,
    };
    HttpResponse::Ok().json(ResultResponse {
        result: core::add(a, b),
    })
}

#[post("/subtract")]
async fn api_subtract(body: web::Json<serde_json::Value>) -> HttpResponse {
    let (a, b) = match parse_and_validate(body) {
        Ok(v) => v,
        Err(resp) => return resp,
    };
    HttpResponse::Ok().json(ResultResponse {
        result: core::subtract(a, b),
    })
}

#[post("/multiply")]
async fn api_multiply(body: web::Json<serde_json::Value>) -> HttpResponse {
    let (a, b) = match parse_and_validate(body) {
        Ok(v) => v,
        Err(resp) => return resp,
    };
    HttpResponse::Ok().json(ResultResponse {
        result: core::multiply(a, b),
    })
}

#[post("/divide")]
async fn api_divide(body: web::Json<serde_json::Value>) -> HttpResponse {
    let (a, b) = match parse_and_validate(body) {
        Ok(v) => v,
        Err(resp) => return resp,
    };
    match core::divide(a, b) {
        Ok(result) => HttpResponse::Ok().json(ResultResponse { result }),
        Err(msg) => HttpResponse::BadRequest().json(ErrorResponse { detail: msg }),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check)
        .service(api_add)
        .service(api_subtract)
        .service(api_multiply)
        .service(api_divide);
}

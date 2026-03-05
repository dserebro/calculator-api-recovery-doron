use actix_web::{get, post, web, HttpResponse};

use crate::core;
use crate::models::{CalculationRequest, ErrorResponse, HealthResponse, ResultResponse};

#[get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok".to_string(),
        version: "0.1.0".to_string(),
    })
}

#[post("/add")]
async fn api_add(req: web::Json<CalculationRequest>) -> HttpResponse {
    let result = core::add(req.a, req.b);
    HttpResponse::Ok().json(ResultResponse { result })
}

#[post("/subtract")]
async fn api_subtract(req: web::Json<CalculationRequest>) -> HttpResponse {
    let result = core::subtract(req.a, req.b);
    HttpResponse::Ok().json(ResultResponse { result })
}

#[post("/multiply")]
async fn api_multiply(req: web::Json<CalculationRequest>) -> HttpResponse {
    let result = core::multiply(req.a, req.b);
    HttpResponse::Ok().json(ResultResponse { result })
}

#[post("/divide")]
async fn api_divide(req: web::Json<CalculationRequest>) -> HttpResponse {
    match core::divide(req.a, req.b) {
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

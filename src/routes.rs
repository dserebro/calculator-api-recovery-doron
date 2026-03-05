use actix_web::{web, HttpResponse};

use crate::core;
use crate::errors::AppError;
use crate::models::{CalculationRequest, HealthResponse, ResultResponse};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(health)
        .service(add)
        .service(subtract)
        .service(multiply)
        .service(divide);
}

#[actix_web::get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok".to_string(),
        version: "0.1.0".to_string(),
    })
}

#[actix_web::post("/add")]
async fn add(req: web::Json<CalculationRequest>) -> HttpResponse {
    let result = core::add(req.a, req.b);
    HttpResponse::Ok().json(ResultResponse { result })
}

#[actix_web::post("/subtract")]
async fn subtract(req: web::Json<CalculationRequest>) -> HttpResponse {
    let result = core::subtract(req.a, req.b);
    HttpResponse::Ok().json(ResultResponse { result })
}

#[actix_web::post("/multiply")]
async fn multiply(req: web::Json<CalculationRequest>) -> HttpResponse {
    let result = core::multiply(req.a, req.b);
    HttpResponse::Ok().json(ResultResponse { result })
}

#[actix_web::post("/divide")]
async fn divide(
    req: web::Json<CalculationRequest>,
) -> Result<HttpResponse, AppError> {
    let result = core::divide(req.a, req.b)
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    Ok(HttpResponse::Ok().json(ResultResponse { result }))
}

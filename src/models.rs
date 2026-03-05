use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CalculationRequest {
    pub a: f64,
    pub b: f64,
}

#[derive(Serialize)]
pub struct ResultResponse {
    pub result: f64,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub detail: String,
}

#[derive(Serialize)]
pub struct ValidationErrorItem {
    #[serde(rename = "type")]
    pub error_type: String,
    pub loc: Vec<serde_json::Value>,
    pub msg: String,
}

#[derive(Serialize)]
pub struct ValidationErrorResponse {
    pub detail: Vec<ValidationErrorItem>,
}

use serde::{Deserialize, Serialize};

/// Request body for arithmetic endpoints: `{ "a": number, "b": number }`.
#[derive(Debug, Deserialize)]
pub struct CalculationRequest {
    pub a: f64,
    pub b: f64,
}

/// Response body for arithmetic endpoints: `{ "result": number }`.
#[derive(Debug, Serialize)]
pub struct ResultResponse {
    pub result: f64,
}

/// Response body for the health check endpoint.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

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
    pub input: serde_json::Value,
}

#[derive(Serialize)]
pub struct ValidationErrorResponse {
    pub detail: Vec<ValidationErrorItem>,
}

pub fn validate_calculation_request(
    body: &serde_json::Value,
) -> Result<(f64, f64), Vec<ValidationErrorItem>> {
    let obj = match body.as_object() {
        Some(obj) => obj,
        None => {
            return Err(vec![ValidationErrorItem {
                error_type: "model_type".to_string(),
                loc: vec![serde_json::Value::String("body".to_string())],
                msg: "Input should be a valid dictionary".to_string(),
                input: body.clone(),
            }]);
        }
    };

    let mut errors = Vec::new();
    let mut a_val = None;
    let mut b_val = None;

    match obj.get("a") {
        None => {
            errors.push(ValidationErrorItem {
                error_type: "missing".to_string(),
                loc: vec![
                    serde_json::Value::String("body".to_string()),
                    serde_json::Value::String("a".to_string()),
                ],
                msg: "Field required".to_string(),
                input: body.clone(),
            });
        }
        Some(v) => match v.as_f64() {
            Some(f) => a_val = Some(f),
            None => {
                errors.push(ValidationErrorItem {
                    error_type: "float_parsing".to_string(),
                    loc: vec![
                        serde_json::Value::String("body".to_string()),
                        serde_json::Value::String("a".to_string()),
                    ],
                    msg: "Input should be a valid number, unable to parse string as a number"
                        .to_string(),
                    input: v.clone(),
                });
            }
        },
    }

    match obj.get("b") {
        None => {
            errors.push(ValidationErrorItem {
                error_type: "missing".to_string(),
                loc: vec![
                    serde_json::Value::String("body".to_string()),
                    serde_json::Value::String("b".to_string()),
                ],
                msg: "Field required".to_string(),
                input: body.clone(),
            });
        }
        Some(v) => match v.as_f64() {
            Some(f) => b_val = Some(f),
            None => {
                errors.push(ValidationErrorItem {
                    error_type: "float_parsing".to_string(),
                    loc: vec![
                        serde_json::Value::String("body".to_string()),
                        serde_json::Value::String("b".to_string()),
                    ],
                    msg: "Input should be a valid number, unable to parse string as a number"
                        .to_string(),
                    input: v.clone(),
                });
            }
        },
    }

    if errors.is_empty() {
        Ok((a_val.unwrap(), b_val.unwrap()))
    } else {
        Err(errors)
    }
}

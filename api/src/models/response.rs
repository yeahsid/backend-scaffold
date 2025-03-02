use std::collections::HashMap;

use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ApiErrorResponse {
    pub message: String,
}

#[derive(Serialize, ToSchema)]
pub struct ValidationErrorResponse<T> {
    pub message: String,
    pub details: T,
}

impl<T> From<T> for ValidationErrorResponse<T> {
    fn from(t: T) -> Self {
        Self {
            message: "Validation error".to_string(),
            details: t,
        }
    }
}

// Define the type alias explicitly
pub type ParamsErrorResponse = ValidationErrorResponse<HashMap<String, Vec<HashMap<String, String>>>>;

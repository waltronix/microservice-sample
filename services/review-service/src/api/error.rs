//! API error mapping for review-service.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

use crate::domain;

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    Validation(String),
    Conflict,
    Infrastructure(String),
}

impl From<domain::Error> for ApiError {
    fn from(value: domain::Error) -> Self {
        match value {
            domain::Error::NotFound => ApiError::NotFound,
            domain::Error::Validation(msg) => ApiError::Validation(msg),
            domain::Error::Conflict => ApiError::Conflict,
            domain::Error::Infrastructure(msg) => ApiError::Infrastructure(msg),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "not found".to_string()),
            ApiError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Conflict => (StatusCode::CONFLICT, "conflict".to_string()),
            ApiError::Infrastructure(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}

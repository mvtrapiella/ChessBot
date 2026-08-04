use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug, PartialEq, Eq)]
pub enum ApiError{
    NotFound,
    BadRequest(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::NotFound => (
                StatusCode::NOT_FOUND, 
                "Resource not found".to_string()
            ),
            ApiError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST, 
                msg
            ),
        };

        // Create a unified JSON body for all errors
        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16()
        }));

        (status, body).into_response()
    }
}
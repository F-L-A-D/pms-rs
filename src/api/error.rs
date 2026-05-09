use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::api::dto::error::ErrorResponse;
use crate::error::app_error::AppError;

#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub message: String,
}

impl ApiError {
    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(ErrorResponse {
                error: self.message,
            }),
        )
            .into_response()
    }
}

pub fn map_app_error(err: AppError) -> ApiError {
    match err {
        AppError::Validation(message) => ApiError::new(StatusCode::BAD_REQUEST, message),

        AppError::NotFound(message) => ApiError::new(StatusCode::NOT_FOUND, message),

        AppError::Conflict(message) => ApiError::new(StatusCode::CONFLICT, message),

        AppError::Infrastructure(_) => {
            ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
        }
    }
}

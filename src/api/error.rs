use axum::http::StatusCode;

use crate::error::app_error::AppError;

pub fn map_app_error(
    err: AppError,
) -> StatusCode {

    match err {

        AppError::Validation(_) =>
            StatusCode::BAD_REQUEST,

        AppError::NotFound(_) =>
            StatusCode::NOT_FOUND,

        AppError::Conflict(_) =>
            StatusCode::CONFLICT,

        AppError::Infrastructure(_) =>
            StatusCode::INTERNAL_SERVER_ERROR,
    }
}
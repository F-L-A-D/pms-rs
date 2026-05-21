use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use uuid::Uuid;

use crate::{
    api::{
        dto::response::semantic_signal::OperationSemanticSignalResponse,
        error::{map_app_error, ApiError},
        state::AppState,
    },
    usecase::semantic_signal::detail::get_operation_semantic_signal,
};

pub async fn get_operation_semantic_signal_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<OperationSemanticSignalResponse>, ApiError> {
    let event_id =
        Uuid::parse_str(&id).map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    let signal = get_operation_semantic_signal::execute(&state.db, event_id)
        .await
        .map_err(map_app_error)?;

    Ok(Json(signal.into()))
}

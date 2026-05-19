use axum::{
    extract::{Path, State},
    Json,
};

use uuid::Uuid;

use crate::{
    api::{
        dto::billing::response::receivable_response::ReceivableResponse,
        error::{map_app_error, ApiError},
        state::AppState,
    },
    error::app_error::validation,
    usecase::billing::detail::get_receivable,
};

pub async fn get_receivable_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ReceivableResponse>, ApiError> {
    let receivable_id = Uuid::parse_str(&id).map_err(|e| map_app_error(validation(e)))?;

    let receivable = get_receivable::execute(&state.db, receivable_id)
        .await
        .map_err(map_app_error)?;

    Ok(Json(receivable.into()))
}

use axum::{
    extract::{Path, State},
    Json,
};

use uuid::Uuid;

use crate::{
    api::{
        dto::billing::response::folio_detail_response::FolioDetailResponse,
        error::{map_app_error, ApiError},
        state::AppState,
    },
    error::app_error::validation,
    usecase::billing::detail::get_folio,
};

pub async fn get_folio_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<FolioDetailResponse>, ApiError> {
    let folio_id = Uuid::parse_str(&id).map_err(|e| map_app_error(validation(e)))?;

    let detail = get_folio::execute(&state.db, folio_id)
        .await
        .map_err(map_app_error)?;

    Ok(Json(detail))
}

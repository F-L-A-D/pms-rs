use axum::{
    extract::{Path, State},
    Json,
};

use uuid::Uuid;

use crate::{
    api::{
        dto::billing::response::folio_response::FolioResponse,
        error::{map_app_error, ApiError},
        state::AppState,
    },
    domain::entity::folio::Folio,
    error::app_error::validation,
    usecase::billing::detail::get_folio,
};

pub async fn get_folio_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<FolioResponse>, ApiError> {
    let folio_id = Uuid::parse_str(&id).map_err(|e| map_app_error(validation(e)))?;

    let folio = get_folio::execute(&state.db, folio_id)
        .await
        .map_err(map_app_error)?;

    Ok(Json(folio.into()))
}

impl From<Folio> for FolioResponse {
    fn from(folio: Folio) -> Self {
        Self {
            id: folio.id,
            reservation_id: folio.reservation_id,
            billing_account_id: folio.billing_account_id,
            status: folio.status,
            created_at: folio.created_at,
        }
    }
}

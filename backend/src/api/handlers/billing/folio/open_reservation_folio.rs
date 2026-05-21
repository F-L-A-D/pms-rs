use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use uuid::Uuid;

use crate::{
    api::{
        dto::billing::{
            input::open_reservation_folio_input::OpenReservationFolioInput,
            response::folio_response::FolioResponse,
        },
        error::{map_app_error, ApiError},
        state::AppState,
    },
    error::app_error::validation,
    usecase::billing::command::folio::open_reservation_folio,
};

pub async fn open_reservation_folio_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<(StatusCode, Json<FolioResponse>), ApiError> {
    let reservation_id = Uuid::parse_str(&id).map_err(|e| map_app_error(validation(e)))?;

    let folio =
        open_reservation_folio::execute(&state.db, OpenReservationFolioInput { reservation_id })
            .await
            .map_err(map_app_error)?;

    Ok((StatusCode::CREATED, Json(folio.into())))
}

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use uuid::Uuid;

use crate::{
    api::{
        dto::billing::{
            input::close_folio_input::CloseFolioInput, response::folio_response::FolioResponse,
        },
        error::{map_app_error, ApiError},
        state::AppState,
    },
    usecase::billing::command::folio::close_folio,
};

pub async fn close_folio_handler(
    State(state): State<AppState>,
    Path(folio_id): Path<Uuid>,
) -> Result<(StatusCode, Json<FolioResponse>), ApiError> {
    let input = CloseFolioInput { folio_id };

    let folio = close_folio::execute(&state.db, input)
        .await
        .map_err(map_app_error)?;

    Ok((StatusCode::OK, Json(FolioResponse::from(folio))))
}

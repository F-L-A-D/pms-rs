use std::str::FromStr;

use axum::{extract::State, http::StatusCode, Json};

use rust_decimal::Decimal;

use uuid::Uuid;

use crate::{
    api::{
        dto::billing::{
            input::create_folio_entry_input::CreateFolioEntryInput,
            request::create_folio_entry_request::CreateFolioEntryRequest,
            response::folio_entry_response::FolioEntryResponse,
        },
        error::{map_app_error, ApiError},
        state::AppState,
    },
    error::app_error::validation,
    usecase::billing::command::create_folio_entry,
};

pub async fn create_folio_entry_handler(
    State(state): State<AppState>,
    Json(req): Json<CreateFolioEntryRequest>,
) -> Result<(StatusCode, Json<FolioEntryResponse>), ApiError> {
    let folio_id = Uuid::parse_str(&req.folio_id).map_err(|e| map_app_error(validation(e)))?;

    let amount = Decimal::from_str(&req.amount).map_err(|e| map_app_error(validation(e)))?;

    let input = CreateFolioEntryInput {
        folio_id,

        entry_type: req.entry_type,

        amount,

        memo: req.memo,
    };

    let entry = create_folio_entry::execute(&state.db, input)
        .await
        .map_err(map_app_error)?;

    let response = FolioEntryResponse {
        id: entry.id,

        folio_id: entry.folio_id,

        entry_type: entry.entry_type,

        amount: entry.amount,

        occurred_at: entry.occurred_at,

        memo: entry.memo,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

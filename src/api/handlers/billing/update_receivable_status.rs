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
    domain::entity::receivable::Receivable,
    error::app_error::validation,
    usecase::billing::command::{
        dispute_receivable, resolve_receivable_dispute, write_off_receivable,
    },
};

pub async fn dispute_receivable_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ReceivableResponse>, ApiError> {
    let receivable_id = Uuid::parse_str(&id).map_err(|e| map_app_error(validation(e)))?;

    let receivable = dispute_receivable::execute(&state.db, receivable_id)
        .await
        .map_err(map_app_error)?;

    Ok(Json(receivable.into()))
}

pub async fn resolve_receivable_dispute_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ReceivableResponse>, ApiError> {
    let receivable_id = Uuid::parse_str(&id).map_err(|e| map_app_error(validation(e)))?;

    let receivable = resolve_receivable_dispute::execute(&state.db, receivable_id)
        .await
        .map_err(map_app_error)?;

    Ok(Json(receivable.into()))
}

pub async fn write_off_receivable_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ReceivableResponse>, ApiError> {
    let receivable_id = Uuid::parse_str(&id).map_err(|e| map_app_error(validation(e)))?;

    let receivable = write_off_receivable::execute(&state.db, receivable_id)
        .await
        .map_err(map_app_error)?;

    Ok(Json(receivable.into()))
}

impl From<Receivable> for ReceivableResponse {
    fn from(receivable: Receivable) -> Self {
        Self {
            id: receivable.id,
            invoice_id: receivable.invoice_id,
            outstanding_amount: receivable.outstanding_amount,
            due_date: receivable.due_date,
            status: receivable.status,
        }
    }
}

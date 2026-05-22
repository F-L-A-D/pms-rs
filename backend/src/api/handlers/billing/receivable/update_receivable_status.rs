use axum::{
    extract::{Path, State},
    Json,
};

use uuid::Uuid;

use crate::{
    api::{
        dto::billing::{
            request::update_receivable_status_request::UpdateReceivableStatusRequest,
            response::receivable_response::ReceivableResponse,
        },
        error::{map_app_error, ApiError},
        state::AppState,
    },
    error::app_error::validation,
    usecase::billing::command::receivable::{
        dispute_receivable,
        resolve_receivable_dispute,
        write_off_receivable,
    },
};

pub async fn dispute_receivable_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<UpdateReceivableStatusRequest>,
) -> Result<Json<ReceivableResponse>, ApiError> {
    let receivable_id =
        Uuid::parse_str(&id)
            .map_err(|e| map_app_error(validation(e)))?;

    let receivable =
        dispute_receivable::execute(
            &state.db,
            receivable_id,
            request.reason,
        )
        .await
        .map_err(map_app_error)?;

    Ok(Json(receivable.into()))
}

pub async fn resolve_receivable_dispute_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<UpdateReceivableStatusRequest>,
) -> Result<Json<ReceivableResponse>, ApiError> {
    let receivable_id =
        Uuid::parse_str(&id)
            .map_err(|e| map_app_error(validation(e)))?;

    let receivable =
        resolve_receivable_dispute::execute(
            &state.db,
            receivable_id,
            request.reason,
        )
        .await
        .map_err(map_app_error)?;

    Ok(Json(receivable.into()))
}

pub async fn write_off_receivable_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<UpdateReceivableStatusRequest>,
) -> Result<Json<ReceivableResponse>, ApiError> {
    let receivable_id =
        Uuid::parse_str(&id)
            .map_err(|e| map_app_error(validation(e)))?;

    let receivable =
        write_off_receivable::execute(
            &state.db,
            receivable_id,
            request.reason,
        )
        .await
        .map_err(map_app_error)?;

    Ok(Json(receivable.into()))
}
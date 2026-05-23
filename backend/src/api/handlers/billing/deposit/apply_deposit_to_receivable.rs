use std::str::FromStr;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use rust_decimal::Decimal;

use uuid::Uuid;

use crate::{
    api::{
        dto::billing::{
            input::apply_deposit_to_receivable_input::ApplyDepositToReceivableInput,
            request::apply_deposit_to_receivable_request::ApplyDepositToReceivableRequest,
            response::deposit_application_response::DepositApplicationResponse,
        },
        error::{map_app_error, ApiError},
        state::AppState,
    },
    error::app_error::validation,
    usecase::billing::command::deposit::apply_deposit_to_receivable,
};

pub async fn apply_deposit_to_receivable_handler(
    State(state): State<AppState>,
    Path(deposit_id): Path<String>,
    Json(req): Json<ApplyDepositToReceivableRequest>,
) -> Result<(StatusCode, Json<DepositApplicationResponse>), ApiError> {
    let deposit_id =
        Uuid::parse_str(&deposit_id)
            .map_err(|e| map_app_error(validation(e)))?;

    let receivable_id =
        Uuid::parse_str(&req.receivable_id)
            .map_err(|e| map_app_error(validation(e)))?;

    let amount =
        Decimal::from_str(&req.amount)
            .map_err(|e| map_app_error(validation(e)))?;

    let result =
        apply_deposit_to_receivable::execute(
            &state.db,
            ApplyDepositToReceivableInput {
                deposit_id,
                receivable_id,
                amount,
                reason: req.reason,
            },
        )
        .await
        .map_err(map_app_error)?;

    Ok((
        StatusCode::CREATED,
        Json(DepositApplicationResponse::from(result.application)),
    ))
}
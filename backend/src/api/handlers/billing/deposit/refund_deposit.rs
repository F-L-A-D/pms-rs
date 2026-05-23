use rust_decimal::Decimal;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use uuid::Uuid;

use crate::{
    api::{
        dto::billing::{
            input::refund_deposit_input::RefundDepositInput,
            request::refund_deposit_request::RefundDepositRequest,
            response::deposit_refund_response::DepositRefundResponse,
        },
        error::{map_app_error, ApiError},
        state::AppState,
    },
    error::app_error::validation,
    usecase::billing::command::deposit::refund_deposit,
};

pub async fn refund_deposit_handler(
    State(state): State<AppState>,
    Path(deposit_id): Path<Uuid>,
    Json(req): Json<RefundDepositRequest>,
) -> Result<(StatusCode, Json<DepositRefundResponse>), ApiError> {
    let amount = req
        .amount
        .parse::<Decimal>()
        .map_err(|_| map_app_error(validation("invalid deposit refund amount")))?;

    let input = RefundDepositInput {
        deposit_id,
        amount,
        reason: req.reason,
    };

    let refund = refund_deposit::execute(&state.db, input)
        .await
        .map_err(map_app_error)?;

    Ok((
        StatusCode::CREATED,
        Json(DepositRefundResponse::from(refund)),
    ))
}

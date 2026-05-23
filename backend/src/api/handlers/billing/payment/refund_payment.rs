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
            input::refund_payment_input::RefundPaymentInput,
            request::refund_payment_request::RefundPaymentRequest,
            response::payment_refund_response::PaymentRefundResponse,
        },
        error::{map_app_error, ApiError},
        state::AppState,
    },
    error::app_error::validation,
    usecase::billing::command::payment::refund_payment,
};

pub async fn refund_payment_handler(
    State(state): State<AppState>,
    Path(payment_id): Path<Uuid>,
    Json(req): Json<RefundPaymentRequest>,
) -> Result<(StatusCode, Json<PaymentRefundResponse>), ApiError> {
    let amount =
        req.amount
            .parse::<Decimal>()
            .map_err(|_| {
                map_app_error(
                    validation("invalid refund amount"),
                )
            })?;

    let input =
        RefundPaymentInput {
            payment_id,
            amount,
            reason: req.reason,
        };

    let refund =
        refund_payment::execute(
            &state.db,
            input,
        )
        .await
        .map_err(map_app_error)?;

    Ok((
        StatusCode::CREATED,
        Json(PaymentRefundResponse::from(refund)),
    ))
}
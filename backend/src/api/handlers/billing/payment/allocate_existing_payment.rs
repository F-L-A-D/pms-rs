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
            input::allocate_existing_payment_input::AllocateExistingPaymentInput,
            request::allocate_existing_payment_request::AllocateExistingPaymentRequest,
            response::payment_allocation_response::PaymentAllocationResponse,
        },
        error::{map_app_error, ApiError},
        state::AppState,
    },
    error::app_error::validation,
    usecase::billing::command::payment::allocate_existing_payment_to_receivable,
};

pub async fn allocate_existing_payment_handler(
    State(state): State<AppState>,
    Path(payment_id): Path<String>,
    Json(req): Json<AllocateExistingPaymentRequest>,
) -> Result<(StatusCode, Json<PaymentAllocationResponse>), ApiError> {
    let payment_id = Uuid::parse_str(&payment_id).map_err(|e| map_app_error(validation(e)))?;

    let receivable_id =
        Uuid::parse_str(&req.receivable_id).map_err(|e| map_app_error(validation(e)))?;

    let amount = Decimal::from_str(&req.amount).map_err(|e| map_app_error(validation(e)))?;

    let result = allocate_existing_payment_to_receivable::execute(
        &state.db,
        AllocateExistingPaymentInput {
            payment_id,
            receivable_id,
            amount,
            reason: req.reason,
        },
    )
    .await
    .map_err(map_app_error)?;

    Ok((
        StatusCode::CREATED,
        Json(PaymentAllocationResponse {
            id: result.allocation.id,
            payment_id: result.allocation.payment_id,
            receivable_id: result.allocation.receivable_id,
            amount: result.allocation.amount,
            allocated_at: result.allocation.allocated_at,
            remaining_outstanding_amount: result.remaining_outstanding_amount,
        }),
    ))
}

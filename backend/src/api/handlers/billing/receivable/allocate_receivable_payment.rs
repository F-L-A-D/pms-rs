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
            input::allocate_receivable_payment_input::AllocateReceivablePaymentInput,
            request::allocate_receivable_payment_request::AllocateReceivablePaymentRequest,
            response::payment_allocation_response::PaymentAllocationResponse,
        },
        error::{map_app_error, ApiError},
        state::AppState,
    },
    error::app_error::validation,
    usecase::billing::command::payment::allocate_receivable_payment,
};

pub async fn allocate_receivable_payment_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<AllocateReceivablePaymentRequest>,
) -> Result<(StatusCode, Json<PaymentAllocationResponse>), ApiError> {
    let receivable_id = Uuid::parse_str(&id).map_err(|e| map_app_error(validation(e)))?;
    let amount = Decimal::from_str(&req.amount).map_err(|e| map_app_error(validation(e)))?;

    let result = allocate_receivable_payment::execute(
        &state.db,
        AllocateReceivablePaymentInput {
            receivable_id,
            amount,
            method: req.method,
            external_reference: req.external_reference,
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

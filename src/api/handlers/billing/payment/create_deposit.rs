use std::str::FromStr;

use axum::{extract::State, http::StatusCode, Json};

use rust_decimal::Decimal;

use uuid::Uuid;

use crate::{
    api::{
        dto::billing::{
            input::create_deposit_input::CreateDepositInput,
            request::create_deposit_request::CreateDepositRequest,
            response::payment_response::PaymentResponse,
        },
        error::{map_app_error, ApiError},
        state::AppState,
    },
    error::app_error::validation,
    usecase::billing::command::create_deposit,
};

pub async fn create_deposit_handler(
    State(state): State<AppState>,
    Json(req): Json<CreateDepositRequest>,
) -> Result<(StatusCode, Json<PaymentResponse>), ApiError> {
    let folio_id = Uuid::parse_str(&req.folio_id).map_err(|e| map_app_error(validation(e)))?;

    let amount = Decimal::from_str(&req.amount).map_err(|e| map_app_error(validation(e)))?;

    let payment = create_deposit::execute(
        &state.db,
        CreateDepositInput {
            folio_id,
            amount,
            method: req.method,
            external_reference: req.external_reference,
        },
    )
    .await
    .map_err(map_app_error)?;

    let response = PaymentResponse {
        id: payment.id,
        folio_id: payment.folio_id,
        amount: payment.amount,
        method: payment.method,
        external_reference: payment.external_reference,
        paid_at: payment.paid_at,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

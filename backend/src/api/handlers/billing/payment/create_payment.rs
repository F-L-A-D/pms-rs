use std::str::FromStr;

use axum::{extract::State, http::StatusCode, Json};

use rust_decimal::Decimal;

use uuid::Uuid;

use crate::{
    api::{
        dto::billing::{
            input::create_payment_input::CreatePaymentInput,
            request::create_payment_request::CreatePaymentRequest,
            response::payment_response::PaymentResponse,
        },
        error::{map_app_error, ApiError},
        state::AppState,
    },
    error::app_error::validation,
    usecase::billing::command::payment::create_payment,
};

pub async fn create_payment_handler(
    State(state): State<AppState>,
    Json(req): Json<CreatePaymentRequest>,
) -> Result<(StatusCode, Json<PaymentResponse>), ApiError> {
    let folio_id = Uuid::parse_str(&req.folio_id).map_err(|e| map_app_error(validation(e)))?;

    let amount = Decimal::from_str(&req.amount).map_err(|e| map_app_error(validation(e)))?;

    let input = CreatePaymentInput {
        folio_id,

        amount,

        method: req.method,

        external_reference: req.external_reference,
    };

    let payment = create_payment::execute(&state.db, input)
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

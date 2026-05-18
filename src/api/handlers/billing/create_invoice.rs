use std::str::FromStr;

use axum::{extract::State, http::StatusCode, Json};

use rust_decimal::Decimal;

use uuid::Uuid;

use crate::{
    api::{
        dto::billing::{
            input::create_invoice_input::CreateInvoiceInput,
            request::create_invoice_request::CreateInvoiceRequest,
            response::invoice_response::InvoiceResponse,
        },
        error::{map_app_error, ApiError},
        state::AppState,
    },
    error::app_error::validation,
    usecase::billing::command::create_invoice,
};

pub async fn create_invoice_handler(
    State(state): State<AppState>,
    Json(req): Json<CreateInvoiceRequest>,
) -> Result<(StatusCode, Json<InvoiceResponse>), ApiError> {
    let folio_id = Uuid::parse_str(&req.folio_id).map_err(|e| map_app_error(validation(e)))?;

    let issued_amount =
        Decimal::from_str(&req.issued_amount).map_err(|e| map_app_error(validation(e)))?;

    let input = CreateInvoiceInput {
        folio_id,

        invoice_number: req.invoice_number,

        issued_amount,

        due_date: req.due_date,
    };

    let invoice = create_invoice::execute(&state.db, input)
        .await
        .map_err(map_app_error)?;

    let response = InvoiceResponse {
        id: invoice.id,

        folio_id: invoice.folio_id,

        billing_account_id: invoice.billing_account_id,

        invoice_number: invoice.invoice_number,

        issued_amount: invoice.issued_amount,

        due_date: invoice.due_date,

        status: invoice.status,

        issued_at: invoice.issued_at,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

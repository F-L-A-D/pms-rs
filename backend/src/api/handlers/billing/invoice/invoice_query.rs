use axum::{
    extract::{Path, State},
    Json,
};

use uuid::Uuid;

use crate::{
    api::{
        dto::billing::response::invoice_response::InvoiceResponse,
        error::{map_app_error, ApiError},
        state::AppState,
    },
    error::app_error::validation,
    usecase::billing::{detail::get_invoice, search::list_billing_account_invoices},
};

pub async fn get_invoice_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<InvoiceResponse>, ApiError> {
    let invoice_id = Uuid::parse_str(&id).map_err(|e| map_app_error(validation(e)))?;

    let invoice = get_invoice::execute(&state.db, invoice_id)
        .await
        .map_err(map_app_error)?;

    Ok(Json(invoice.into()))
}

pub async fn list_billing_account_invoices_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Vec<InvoiceResponse>>, ApiError> {
    let billing_account_id = Uuid::parse_str(&id).map_err(|e| map_app_error(validation(e)))?;

    let invoices = list_billing_account_invoices::execute(&state.db, billing_account_id)
        .await
        .map_err(map_app_error)?;

    Ok(Json(
        invoices.into_iter().map(InvoiceResponse::from).collect(),
    ))
}

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
    usecase::billing::command::void_invoice,
};

pub async fn void_invoice_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<InvoiceResponse>, ApiError> {
    let invoice_id = Uuid::parse_str(&id).map_err(|e| map_app_error(validation(e)))?;

    let invoice = void_invoice::execute(&state.db, invoice_id)
        .await
        .map_err(map_app_error)?;

    Ok(Json(invoice.into()))
}

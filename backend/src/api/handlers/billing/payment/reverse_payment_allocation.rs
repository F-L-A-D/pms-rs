use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use uuid::Uuid;

use crate::{
    api::{
        dto::billing::request::reverse_payment_allocation_request::ReversePaymentAllocationRequest,
        error::{map_app_error, ApiError},
        state::AppState,
    },
    error::app_error::validation,
    usecase::billing::command::payment::reverse_payment_allocation,
};

pub async fn reverse_payment_allocation_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<ReversePaymentAllocationRequest>,
) -> Result<StatusCode, ApiError> {
    let allocation_id = Uuid::parse_str(&id).map_err(|e| map_app_error(validation(e)))?;

    reverse_payment_allocation::execute(&state.db, allocation_id, request.reason)
        .await
        .map_err(map_app_error)?;

    Ok(StatusCode::NO_CONTENT)
}

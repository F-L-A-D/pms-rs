use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{
    api::{
        dto::billing::response::billing_audit_response::BillingAuditResponse,
        error::{map_app_error, ApiError},
        state::AppState,
    },
    usecase::billing::detail::get_folio_audit,
};

pub async fn get_folio_audit_handler(
    State(state): State<AppState>,
    Path(folio_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Vec<BillingAuditResponse>>), ApiError> {
    let response = get_folio_audit::execute(&state.db, folio_id)
        .await
        .map_err(map_app_error)?;

    Ok((StatusCode::OK, Json(response)))
}

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use uuid::Uuid;

use crate::{
    api::{
        dto::billing::{
            input::assign_billing_account_input::AssignBillingAccountInput,
            request::assign_billing_account_request::AssignBillingAccountRequest,
            response::folio_response::FolioResponse,
        },
        error::{map_app_error, ApiError},
        state::AppState,
    },
    error::app_error::validation,
    usecase::billing::command::account::assign_billing_account,
};

pub async fn assign_billing_account_handler(
    State(state): State<AppState>,
    Path(folio_id): Path<Uuid>,
    Json(req): Json<AssignBillingAccountRequest>,
) -> Result<(StatusCode, Json<FolioResponse>), ApiError> {
    let billing_account_id =
        Uuid::parse_str(&req.billing_account_id).map_err(|e| map_app_error(validation(e)))?;

    let input = AssignBillingAccountInput {
        folio_id,
        billing_account_id,
    };

    let folio = assign_billing_account::execute(&state.db, input)
        .await
        .map_err(map_app_error)?;

    let response = FolioResponse {
        id: folio.id,
        reservation_id: folio.reservation_id,
        billing_account_id: folio.billing_account_id,
        status: folio.status,
        created_at: folio.created_at,
    };

    Ok((StatusCode::OK, Json(response)))
}

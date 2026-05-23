use axum::{extract::State, http::StatusCode, Json};

use crate::{
    api::{
        dto::billing::{
            request::create_billing_account_request::CreateBillingAccountRequest,
            response::billing_account_response::BillingAccountResponse,
        },
        error::{map_app_error, ApiError},
        state::AppState,
    },
    usecase::billing::command::account::create_billing_account,
};

pub async fn create_billing_account_handler(
    State(state): State<AppState>,
    Json(request): Json<CreateBillingAccountRequest>,
) -> Result<(StatusCode, Json<BillingAccountResponse>), ApiError> {
    let account = create_billing_account::execute(&state.db, request.into())
        .await
        .map_err(map_app_error)?;

    Ok((
        StatusCode::CREATED,
        Json(BillingAccountResponse::from(account)),
    ))
}

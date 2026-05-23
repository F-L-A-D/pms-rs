use axum::{extract::State, http::StatusCode, Json};

use rust_decimal::Decimal;

use uuid::Uuid;

use crate::{
    api::{
        dto::billing::{
            input::create_deposit_input::CreateDepositInput,
            request::create_deposit_request::CreateDepositRequest,
            response::deposit_response::DepositResponse,
        },
        error::{map_app_error, ApiError},
        state::AppState,
    },
    error::app_error::validation,
    usecase::billing::command::deposit::create_deposit,
};

pub async fn create_deposit_handler(
    State(state): State<AppState>,
    Json(request): Json<CreateDepositRequest>,
) -> Result<(StatusCode, Json<DepositResponse>), ApiError> {
    let folio_id = Uuid::parse_str(&request.folio_id).map_err(|e| map_app_error(validation(e)))?;

    let amount = request
        .amount
        .parse::<Decimal>()
        .map_err(|e| map_app_error(validation(e)))?;

    let deposit = create_deposit::execute(
        &state.db,
        CreateDepositInput {
            folio_id,
            amount,
            method: request.method,
            external_reference: request.external_reference,
            reason: request.reason,
        },
    )
    .await
    .map_err(map_app_error)?;

    Ok((StatusCode::CREATED, Json(DepositResponse::from(deposit))))
}

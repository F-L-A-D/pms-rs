use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use uuid::Uuid;

use crate::{
    api::{
        dto::billing::{
            input::reverse_deposit_application_input::ReverseDepositApplicationInput,
            request::reverse_deposit_application_request::ReverseDepositApplicationRequest,
        },
        error::{map_app_error, ApiError},
        state::AppState,
    },
    error::app_error::validation,
    usecase::billing::command::deposit::reverse_deposit_application,
};

pub async fn reverse_deposit_application_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<ReverseDepositApplicationRequest>,
) -> Result<StatusCode, ApiError> {
    let deposit_application_id =
        Uuid::parse_str(&id)
            .map_err(|e| map_app_error(validation(e)))?;

    reverse_deposit_application::execute(
        &state.db,
        ReverseDepositApplicationInput {
            deposit_application_id,
            reason: request.reason,
        },
    )
    .await
    .map_err(map_app_error)?;

    Ok(StatusCode::NO_CONTENT)
}
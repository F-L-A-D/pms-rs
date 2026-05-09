use axum::{
    extract::{Path, State},
    Json,
};

use uuid::Uuid;

use crate::api::dto::guest_summary::GuestSummaryResponse;
use crate::api::error::{map_app_error, ApiError};
use crate::api::state::AppState;

use crate::error::app_error::AppError;

use crate::usecase::timeline::get_guest_summary::get_guest_summary;

pub async fn get_guest_summary_handler(
    State(state): State<AppState>,
    Path(guest_id): Path<String>,
) -> Result<Json<GuestSummaryResponse>, ApiError> {
    let mut tx = state.db.begin_tx().await;

    let guest_id = Uuid::parse_str(&guest_id)
        .map_err(|e| map_app_error(AppError::Validation(e.to_string())))?;

    let metrics = get_guest_summary(&mut tx, guest_id)
        .await
        .map_err(map_app_error)?;

    tx.rollback()
        .await
        .map_err(|e| map_app_error(AppError::Infrastructure(e.to_string())))?;

    Ok(Json(GuestSummaryResponse {
        total_stays: metrics.total_stays,

        total_nights: metrics.total_nights,

        total_spending: metrics.total_spending,

        last_stay_at: metrics.last_stay_at,
    }))
}

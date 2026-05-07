use axum::{
    extract::{
        Path,
        State,
    },
    http::StatusCode,
    Json,
};

use crate::api::dto::guest_metrics::GuestMetricsResponse;

use crate::api::error::map_app_error;

use crate::api::state::AppState;

use crate::usecase::timeline::get_guest_metrics::get_guest_metrics;

pub async fn get_guest_metrics_handler(
    State(state): State<AppState>,
    Path(guest_id): Path<String>,
) -> Result<Json<GuestMetricsResponse>, StatusCode> {

    let metrics =
        get_guest_metrics(
            &state.db,
            &guest_id,
        )
        .await
        .map_err(map_app_error)?;

    Ok(
        Json(
            GuestMetricsResponse {
                total_stays:
                    metrics.total_stays,

                total_nights:
                    metrics.total_nights,

                total_spending:
                    metrics.total_spending,

                last_stay_at:
                    metrics.last_stay_at,
            }
        )
    )
}
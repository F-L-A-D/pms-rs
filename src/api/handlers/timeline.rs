use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use uuid::Uuid;

use crate::{
    api::{
        dto::timeline::TimelineEventResponse,
        error::{map_app_error, ApiError},
        state::AppState,
    },
    usecase::timeline::search::get_guest_timelines::get_guest_timelines,
};

pub async fn get_guest_timelines_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Vec<TimelineEventResponse>>, ApiError> {
    let guest_id =
        Uuid::parse_str(&id).map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    let events = get_guest_timelines(&state.db, guest_id)
        .await
        .map_err(map_app_error)?;

    let response = events.into_iter().map(Into::into).collect();

    Ok(Json(response))
}

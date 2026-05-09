use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use uuid::Uuid;

use crate::api::dto::timeline::{CreateTimelineEventRequest, TimelineEventResponse};

use crate::api::error::ApiError;
use crate::api::state::AppState;

use crate::domain::guest_timeline_event::{GuestTimelineEvent, TimelineEventType};

use crate::repository::sqlite::operational::guest_timeline_event_repository::SqliteGuestTimelineEventRepository;

use crate::usecase::timeline::list_guest_timeline::list_guest_timeline;

pub async fn create_timeline_event_handler(
    State(state): State<AppState>,
    Json(req): Json<CreateTimelineEventRequest>,
) -> Result<StatusCode, ApiError> {
    let event_type = match req.event_type.as_str() {
        "ReservationCreated" => TimelineEventType::ReservationCreated,

        "CheckedIn" => TimelineEventType::CheckedIn,

        "RoomChargePosted" => TimelineEventType::RoomChargePosted,

        _ => {
            return Err(ApiError::new(
                StatusCode::BAD_REQUEST,
                "invalid timeline event type",
            ))
        }
    };

    let event = GuestTimelineEvent::new(req.id, req.guest_id, event_type, req.reference_id);

    let mut tx = state.db.begin_tx().await;

    SqliteGuestTimelineEventRepository::save(&mut tx, &event)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "internal server error"))?;

    tx.commit()
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "internal server error"))?;

    Ok(StatusCode::OK)
}

pub async fn get_guest_timeline_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Vec<TimelineEventResponse>>, ApiError> {
    let guest_id =
        Uuid::parse_str(&id).map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    let events = list_guest_timeline(&state.db, guest_id)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "internal server error"))?;

    let response = events
        .into_iter()
        .map(|e| TimelineEventResponse {
            event_type: format!("{:?}", e.event_type),

            reference_id: e.reference_id,

            occurred_at: e.occurred_at,
        })
        .collect();

    Ok(Json(response))
}

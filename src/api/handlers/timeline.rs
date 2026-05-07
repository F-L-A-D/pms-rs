use axum::{
    extract::{
        Path,
        State,
    },
    http::StatusCode,
    Json,
};

use crate::api::dto::timeline::{
    CreateTimelineEventRequest,
    TimelineEventResponse,
};

use crate::api::state::AppState;

use crate::domain::guest_timeline_event::{
    GuestTimelineEvent,
    TimelineEventType,
};

use crate::repository::sqlite::guest_timeline_event_repository::SqliteGuestTimelineEventRepository;

use crate::usecase::timeline::list_guest_timeline::list_guest_timeline;

pub async fn create_timeline_event_handler(
    State(state): State<AppState>,
    Json(req): Json<CreateTimelineEventRequest>,
) -> Result<StatusCode, StatusCode> {

    let event_type =
        match req.event_type.as_str() {

            "ReservationCreated" =>
                TimelineEventType::ReservationCreated,

            "CheckedIn" =>
                TimelineEventType::CheckedIn,

            "RoomChargePosted" =>
                TimelineEventType::RoomChargePosted,

            _ =>
                return Err(
                    StatusCode::BAD_REQUEST
                ),
        };

    let event =
        GuestTimelineEvent::new(
            req.id,
            req.guest_id,
            event_type,
            req.reference_id,
        );

    let mut tx =
        state.db.begin_tx().await;

    SqliteGuestTimelineEventRepository::save(
        &mut tx,
        &event,
    )
    .await
    .map_err(|_| {
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tx.commit()
        .await
        .map_err(|_| {
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::OK)
}

pub async fn get_guest_timeline_handler(
    State(state): State<AppState>,
    Path(guest_id): Path<String>,
) -> Result<Json<Vec<TimelineEventResponse>>, StatusCode> {

    let events =
        list_guest_timeline(
            &state.db,
            &guest_id,
        )
        .await
        .map_err(|_| {
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let response =
        events
            .into_iter()
            .map(|e| {

                TimelineEventResponse {
                    event_type:
                        format!("{:?}", e.event_type),

                    reference_id:
                        e.reference_id,

                    occurred_at:
                        e.occurred_at,
                }

            })
            .collect();

    Ok(Json(response))
}
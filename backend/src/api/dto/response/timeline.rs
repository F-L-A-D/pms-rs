use serde::Serialize;

use chrono::{DateTime, Utc};

use uuid::Uuid;

#[derive(Serialize)]
pub struct TimelineEventResponse {
    pub event_type: String,
    pub reference_id: Uuid,
    pub occurred_at: DateTime<Utc>,
}

use crate::domain::semantic::guest_timeline_event::{GuestTimelineEvent, TimelineEventType};

impl From<GuestTimelineEvent> for TimelineEventResponse {
    fn from(event: GuestTimelineEvent) -> Self {
        Self {
            event_type: match event.event_type {
                TimelineEventType::ReservationCreated => "ReservationCreated".into(),

                TimelineEventType::ReservationModified => "ReservationModified".into(),

                TimelineEventType::ReservationNoShow => "ReservationNoShow".into(),

                TimelineEventType::ReservationReinstated => "ReservationReinstated".into(),

                TimelineEventType::ReservationDatesChanged => "ReservationDatesChanged".into(),

                TimelineEventType::ReservationExtended => "ReservationExtended".into(),

                TimelineEventType::ReservationShortened => "ReservationShortened".into(),

                TimelineEventType::ReservationRoomClassChanged => {
                    "ReservationRoomClassChanged".into()
                }

                TimelineEventType::CheckedIn => "CheckedIn".into(),

                TimelineEventType::CheckedOut => "CheckedOut".into(),

                TimelineEventType::RoomMoved => "RoomMoved".into(),

                TimelineEventType::RoomChargePosted => "RoomChargePosted".into(),

                TimelineEventType::ReservationCancelled => "ReservationCancelled".into(),
            },

            reference_id: event.reference_id,

            occurred_at: event.occurred_at,
        }
    }
}

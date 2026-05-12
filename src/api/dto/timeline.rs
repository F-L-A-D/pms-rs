use serde::{
    Deserialize,
    Serialize,
};

use chrono::{
    DateTime,
    Utc,
};

use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateTimelineEventRequest {
    pub id: Uuid,
    pub guest_id: Uuid,
    pub event_type: String,
    pub reference_id: Uuid,
}

#[derive(Serialize)]
pub struct TimelineEventResponse {
    pub event_type: String,
    pub reference_id: Uuid,
    pub occurred_at: DateTime<Utc>,
}

use crate::domain::guest_timeline_event::{
    GuestTimelineEvent,
    TimelineEventType,
};

impl From<GuestTimelineEvent>
    for TimelineEventResponse
{
    fn from(
        event: GuestTimelineEvent,
    ) -> Self {

        Self {
            event_type:
                match event.event_type {

                    TimelineEventType
                        ::ReservationCreated =>
                    {
                        "ReservationCreated"
                            .into()
                    }

                    TimelineEventType
                        ::CheckedIn =>
                    {
                        "CheckedIn"
                            .into()
                    }

                    TimelineEventType
                        ::CheckedOut =>
                    {
                        "CheckedOut"
                            .into()
                    }

                    TimelineEventType
                        ::RoomChargePosted =>
                    {
                        "RoomChargePosted"
                            .into()
                    }

                    TimelineEventType
                        ::ReservationCancelled =>
                    {
                        "ReservationCancelled"
                            .into()
                    }
                },

            reference_id:
                event.reference_id,

            occurred_at:
                event.occurred_at,
        }
    }
}
use chrono::{
    DateTime,
    Utc,
};

use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum TimelineEventType {
    ReservationCreated,
    ReservationCancelled,
    CheckedIn,
    CheckedOut,
    RoomChargePosted,
}

#[derive(Debug, Clone)]
pub struct GuestTimelineEvent {
    pub id: Uuid,
    pub guest_id: Uuid,
    pub event_type: TimelineEventType,
    pub reference_id: Uuid,
    pub occurred_at: DateTime<Utc>,
}

impl GuestTimelineEvent {

    pub fn new(
        id: Uuid,
        guest_id: Uuid,
        event_type: TimelineEventType,
        reference_id: Uuid,
    ) -> Self {

        Self {
            id,
            guest_id,
            event_type,
            reference_id,
            occurred_at: Utc::now(),
        }
    }
}
use chrono::{
    DateTime,
    Utc,
};

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
    pub id: String,
    pub guest_id: String,
    pub event_type: TimelineEventType,
    pub reference_id: String,
    pub occurred_at: DateTime<Utc>,
}

impl GuestTimelineEvent {

    pub fn new(
        id: String,
        guest_id: String,
        event_type: TimelineEventType,
        reference_id: String,
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
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
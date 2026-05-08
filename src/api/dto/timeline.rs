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
    pub id: String,
    pub guest_id: Uuid,
    pub event_type: String,
    pub reference_id: String,
}

#[derive(Serialize)]
pub struct TimelineEventResponse {
    pub event_type: String,
    pub reference_id: String,
    pub occurred_at: DateTime<Utc>,
}
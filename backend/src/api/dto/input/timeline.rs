use serde::Deserialize;

use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateTimelineEventInput {
    pub id: Uuid,
    pub guest_id: Uuid,
    pub event_type: String,
    pub reference_id: Uuid,
}

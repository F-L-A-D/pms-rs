use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReservationEditSession {
    pub id: Uuid,
    pub reservation_id: Uuid,
    pub actor_id: String,
    pub actor_label: Option<String>,
    pub opened_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
}

impl ReservationEditSession {
    pub fn is_active_at(&self, now: DateTime<Utc>) -> bool {
        self.closed_at.is_none() && self.expires_at > now
    }
}

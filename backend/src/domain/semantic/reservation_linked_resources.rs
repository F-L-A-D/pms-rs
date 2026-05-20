use serde::{Deserialize, Serialize,};

use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize,)]
pub struct ReservationLinkedResources {
    pub primary_guest_id: Option<Uuid>,
    pub assigned_room_id: Option<Uuid>,
    pub folio_id: Option<Uuid>,
}
use chrono::{
    DateTime,
    Utc,
};

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct GuestActivitySignal {
    pub guest_id: Uuid,
    pub is_active: bool,
    pub projection_version: i32,
    pub updated_at: DateTime<Utc>,
}

impl PartialEq for GuestActivitySignal {

    fn eq(
        &self,
        other: &Self,
    ) -> bool {

        self.guest_id
            == other.guest_id

        &&

        self.is_active
            == other.is_active

        &&

        self.projection_version
            == other.projection_version
    }
}
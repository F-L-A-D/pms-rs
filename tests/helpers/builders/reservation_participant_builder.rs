use uuid::Uuid;

use serde_json::{
    json,
    Value,
};

pub struct ReservationParticipantBuilder {
    guest_id: Uuid,

    relation_type: String,
}

impl ReservationParticipantBuilder {

    pub fn primary(
        guest_id: Uuid,
    ) -> Self {

        Self {
            guest_id,

            relation_type:
                "Primary".into(),
        }
    }

    pub fn accompany(
        guest_id: Uuid,
    ) -> Self {

        Self {
            guest_id,

            relation_type:
                "Accompany".into(),
        }
    }

    pub fn build(self) -> Value {

        json!({
            "guest_id":
                self.guest_id,

            "relation_type":
                self.relation_type,
        })
    }
}
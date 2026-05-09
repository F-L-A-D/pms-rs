use uuid::Uuid;

use serde_json::{
    json,
    Value,
};

pub struct ReservationRequestBuilder {
    external_id: String,
    check_in: String,
    check_out: String,
    room_class: String,
    participants: Vec<Value>,
}

impl ReservationRequestBuilder {

    pub fn new() -> Self {

        Self {

            external_id:
                "res-1".into(),

            check_in:
                "2026-05-10".into(),

            check_out:
                "2026-05-12".into(),

            room_class:
                "STD".into(),

            participants:
                vec![],
        }
    }

    pub fn with_external_id(
        mut self,
        external_id: &str,
    ) -> Self {

        self.external_id =
            external_id.into();

        self
    }

    pub fn with_dates(
        mut self,
        check_in: &str,
        check_out: &str,
    ) -> Self {

        self.check_in =
            check_in.into();

        self.check_out =
            check_out.into();

        self
    }

    pub fn with_room_class(
        mut self,
        room_class: &str,
    ) -> Self {

        self.room_class =
            room_class.into();

        self
    }

    pub fn with_primary_guest(
        mut self,
        guest_id: Uuid,
    ) -> Self {

        self.participants.push(
            json!({
                "guest_id":
                    guest_id,

                "relation_type":
                    "Primary"
            })
        );

        self
    }

    pub fn with_accompany_guest(
        mut self,
        guest_id: Uuid,
    ) -> Self {

        self.participants.push(
            json!({
                "guest_id":
                    guest_id,

                "relation_type":
                    "Accompany"
            })
        );

        self
    }

    pub fn with_participant(
        mut self,
        participant: Value,
    ) -> Self {

        self.participants.push(
            participant
        );

        self
    }

    pub fn build(self) -> Value {

        json!({
            "external_id":
                self.external_id,

            "check_in":
                self.check_in,

            "check_out":
                self.check_out,

            "room_class":
                self.room_class,

            "participants":
                self.participants,
        })
    }
}
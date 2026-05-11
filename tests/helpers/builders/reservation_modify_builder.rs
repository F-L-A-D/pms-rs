use serde_json::{
    json,
    Value,
};

pub struct ReservationModifyBuilder {

    room_class: Option<String>,

    check_in: Option<String>,

    check_out: Option<String>,
}

impl ReservationModifyBuilder {

    pub fn new() -> Self {

        Self {

            room_class:
                None,

            check_in:
                None,

            check_out:
                None,
        }
    }

    pub fn with_room_class(
        mut self,
        room_class: &str,
    ) -> Self {

        self.room_class =
            Some(
                room_class.into()
            );

        self
    }

    #[allow(dead_code)]
    pub fn with_dates(
        mut self,
        check_in: &str,
        check_out: &str,
    ) -> Self {

        self.check_in =
            Some(
                check_in.into()
            );

        self.check_out =
            Some(
                check_out.into()
            );

        self
    }

    pub fn build(self) -> Value {

        json!({

            "room_class":
                self.room_class,

            "check_in":
                self.check_in,

            "check_out":
                self.check_out,
        })
    }
}
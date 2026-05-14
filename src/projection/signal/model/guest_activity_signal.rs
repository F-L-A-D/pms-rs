use uuid::Uuid;

#[derive(
    Debug,
    Clone,
)]
pub struct GuestActivitySignal {
    guest_id: Uuid,
    active: bool,
}

impl GuestActivitySignal {

    pub fn new(
        guest_id: Uuid,
        active: bool,
    ) -> Self
    {
        Self {
            guest_id,
            active,
        }
    }

    pub fn guest_id(
        &self,
    ) -> Uuid
    {
        self.guest_id
    }

    pub fn active(
        &self,
    ) -> bool
    {
        self.active
    }
}
use uuid::Uuid;

#[derive(Debug)]
pub struct GuestActivitySource {
    pub guest_id: Uuid,
}
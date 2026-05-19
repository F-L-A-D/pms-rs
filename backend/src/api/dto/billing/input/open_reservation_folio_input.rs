use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct OpenReservationFolioInput {
    pub reservation_id: Uuid,
}

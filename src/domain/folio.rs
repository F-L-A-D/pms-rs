use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum FolioStatus {
    Open,
    Closed,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Folio {
    pub id: Uuid,
    pub reservation_id: Uuid,
    pub status: FolioStatus,
}

impl Folio {
    pub fn new(id: Uuid, reservation_id: Uuid) -> Self {
        Self {
            id,
            reservation_id,
            status: FolioStatus::Open,
        }
    }

    pub fn close(&mut self) -> Result<(), String> {
        if self.status == FolioStatus::Closed {
            return Err("folio already closed".into());
        }
        self.status = FolioStatus::Closed;

        Ok(())
    }
}

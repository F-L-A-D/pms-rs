use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CloseFolioInput {
    pub folio_id: Uuid,
}

use chrono::{DateTime, Utc};

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum FolioEntryType {
    RoomCharge,
    Payment,
    Adjustment,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FolioEntry {
    pub id: Uuid,
    pub folio_id: Uuid,
    pub entry_type: FolioEntryType,
    pub amount: i64,
    pub occurred_at: DateTime<Utc>,
    pub description: Option<String>,
}

impl FolioEntry {
    pub fn new(
        id: Uuid,
        folio_id: Uuid,
        entry_type: FolioEntryType,
        amount: i64,
        description: Option<String>,
    ) -> Self {
        Self {
            id,
            folio_id,
            entry_type,
            amount,
            occurred_at: Utc::now(),
            description,
        }
    }
}


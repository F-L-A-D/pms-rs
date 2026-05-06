use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub enum EntryType {
    RoomCharge,
    Payment,
    Adjustment,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FolioEntry {
    pub id: String,
    pub folio_id: String,
    pub entry_type: EntryType,
    pub amount: i64,
    pub occurred_at: DateTime<Utc>,
    pub description: Option<String>,
}

impl FolioEntry {
    pub fn new(
        id: String,
        folio_id: String,
        entry_type: EntryType,
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


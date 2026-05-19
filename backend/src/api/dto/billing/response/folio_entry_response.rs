use chrono::{DateTime, Utc};

use rust_decimal::Decimal;

use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::domain::entity::folio_entry::FolioEntryType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolioEntryResponse {
    pub id: Uuid,
    pub folio_id: Uuid,
    pub entry_type: FolioEntryType,
    pub amount: Decimal,
    pub occurred_at: DateTime<Utc>,
    pub memo: Option<String>,
}

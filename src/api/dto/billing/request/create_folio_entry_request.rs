use serde::{Deserialize, Serialize};

use crate::domain::entity::folio_entry::FolioEntryType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFolioEntryRequest {
    pub folio_id: String,
    pub entry_type: FolioEntryType,
    pub amount: String,
    pub memo: Option<String>,
}

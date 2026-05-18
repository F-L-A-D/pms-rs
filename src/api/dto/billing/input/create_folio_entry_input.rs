use rust_decimal::Decimal;

use uuid::Uuid;

use crate::domain::entity::folio_entry::FolioEntryType;

#[derive(Debug, Clone)]
pub struct CreateFolioEntryInput {
    pub folio_id: Uuid,
    pub entry_type: FolioEntryType,
    pub amount: Decimal,
    pub memo: Option<String>,
}

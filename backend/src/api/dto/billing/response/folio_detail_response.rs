use rust_decimal::Decimal;

use serde::{Deserialize, Serialize};

use crate::api::dto::billing::response::{
    folio_entry_response::FolioEntryResponse, folio_response::FolioResponse,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolioDetailResponse {
    pub folio: FolioResponse,

    pub entries: Vec<FolioEntryResponse>,

    pub total_charges: Decimal,

    pub total_payments: Decimal,

    pub balance: Decimal,
}

use serde::{
    Deserialize,
    Serialize,
};

use chrono::{
    DateTime,
    Utc,
};

#[derive(Deserialize)]
pub struct OpenFolioRequest {
    pub folio_id: String,
    pub reservation_id: String,
}

#[derive(Deserialize)]
pub struct PostRoomChargeRequest {
    pub amount: i64,
    pub description: String,
}

#[derive(Deserialize)]
pub struct PostPaymentRequest {
    pub amount: i64,
    pub description: String,
}

#[derive(Serialize)]
pub struct FolioResponse {
    pub folio_id: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct BalanceResponse {
    pub folio_id: String,
    pub balance: i64,
}

#[derive(Serialize)]
pub struct FolioEntryResponse {
    pub id: String,
    pub entry_type: String,
    pub amount: i64,
    pub description: Option<String>,
    pub occurred_at: DateTime<Utc>,
}
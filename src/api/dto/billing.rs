use serde::{Deserialize, Serialize};

use chrono::{DateTime, Utc};

use uuid::Uuid;

#[derive(Deserialize)]
pub struct OpenFolioRequest {
    pub reservation_id: Uuid,
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
    pub folio_id: Uuid,
    pub status: String,
}

#[derive(Serialize)]
pub struct BalanceResponse {
    pub folio_id: Uuid,
    pub balance: i64,
}

#[derive(Serialize)]
pub struct FolioEntryResponse {
    pub id: Uuid,
    pub entry_type: String,
    pub amount: i64,
    pub description: Option<String>,
    pub occurred_at: DateTime<Utc>,
}

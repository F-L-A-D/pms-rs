use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::domain::entity::folio::FolioStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolioResponse {
    pub id: Uuid,
    pub reservation_id: Uuid,
    pub billing_account_id: Option<Uuid>,
    pub status: FolioStatus,
    pub created_at: DateTime<Utc>,
}

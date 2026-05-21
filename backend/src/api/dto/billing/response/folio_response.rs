use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::domain::entity::folio::{Folio, FolioStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolioResponse {
    pub id: Uuid,
    pub reservation_id: Uuid,
    pub billing_account_id: Option<Uuid>,
    pub status: FolioStatus,
    pub created_at: DateTime<Utc>,
}

impl From<Folio> for FolioResponse {
    fn from(folio: Folio) -> Self {
        Self {
            id: folio.id,
            reservation_id: folio.reservation_id,
            billing_account_id: folio.billing_account_id,
            status: folio.status,
            created_at: folio.created_at,
        }
    }
}

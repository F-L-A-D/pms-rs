use chrono::{DateTime, NaiveDate, Utc};

use rust_decimal::Decimal;

use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NightAuditRoomChargePosting {
    pub id: Uuid,
    pub business_date_id: Uuid,
    pub business_date: NaiveDate,
    pub reservation_id: Uuid,
    pub folio_id: Uuid,
    pub service_date: NaiveDate,
    pub amount: Decimal,
    pub folio_entry_id: Uuid,
    pub posted_at: DateTime<Utc>,
}

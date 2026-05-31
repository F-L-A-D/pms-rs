use chrono::{DateTime, NaiveDate, Utc};

use serde::Serialize;

use uuid::Uuid;

use crate::domain::entity::business_date::{BusinessDate, BusinessDateStatus};

#[derive(Clone, Debug, Serialize)]
pub struct BusinessDateResponse {
    pub id: Uuid,
    pub business_date: NaiveDate,
    pub status: BusinessDateStatus,
    pub opened_at: DateTime<Utc>,
    pub closing_started_at: Option<DateTime<Utc>>,
    pub closed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<BusinessDate> for BusinessDateResponse {
    fn from(value: BusinessDate) -> Self {
        Self {
            id: value.id,
            business_date: value.business_date,
            status: value.status,
            opened_at: value.opened_at,
            closing_started_at: value.closing_started_at,
            closed_at: value.closed_at,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

use chrono::{DateTime, NaiveDate, Utc};

use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::error::app_error::{domain, AppResult};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BusinessDateStatus {
    Open,
    Closing,
    Closed,
}

impl BusinessDateStatus {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Closing => "closing",
            Self::Closed => "closed",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "open" => Some(Self::Open),
            "closing" => Some(Self::Closing),
            "closed" => Some(Self::Closed),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BusinessDate {
    pub id: Uuid,
    pub business_date: NaiveDate,
    pub status: BusinessDateStatus,
    pub opened_at: DateTime<Utc>,
    pub closing_started_at: Option<DateTime<Utc>>,
    pub closed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl BusinessDate {
    pub fn new_open(business_date: NaiveDate, now: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::new_v4(),
            business_date,
            status: BusinessDateStatus::Open,
            opened_at: now,
            closing_started_at: None,
            closed_at: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn start_closing(&mut self, now: DateTime<Utc>) -> AppResult<()> {
        if self.status != BusinessDateStatus::Open {
            return Err(domain("business date must be open to start night audit"));
        }

        self.status = BusinessDateStatus::Closing;
        self.closing_started_at = Some(now);
        self.updated_at = now;

        Ok(())
    }

    pub fn finalize_close(&mut self, now: DateTime<Utc>) -> AppResult<()> {
        if self.status != BusinessDateStatus::Closing {
            return Err(domain(
                "business date must be closing to finalize night audit",
            ));
        }

        self.status = BusinessDateStatus::Closed;
        self.closed_at = Some(now);
        self.updated_at = now;

        Ok(())
    }

    pub fn is_open(&self) -> bool {
        self.status == BusinessDateStatus::Open
    }

    pub fn is_closing(&self) -> bool {
        self.status == BusinessDateStatus::Closing
    }

    pub fn is_closed(&self) -> bool {
        self.status == BusinessDateStatus::Closed
    }
}

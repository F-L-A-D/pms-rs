use chrono::{DateTime, NaiveDate, Utc};

use rust_decimal::Decimal;

use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceStatus {
    Draft,
    Issued,
    Voided,
}

impl InvoiceStatus {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Issued => "issued",
            Self::Voided => "voided",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "draft" => Some(Self::Draft),
            "issued" => Some(Self::Issued),
            "voided" => Some(Self::Voided),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: Uuid,

    pub folio_id: Uuid,

    pub billing_account_id: Uuid,

    pub invoice_number: String,

    pub issued_amount: Decimal,

    pub due_date: NaiveDate,

    pub status: InvoiceStatus,

    pub issued_at: DateTime<Utc>,
}

impl Invoice {
    pub fn void(&mut self) {
        self.status = InvoiceStatus::Voided;
    }
}

use chrono::{DateTime, Utc};

use rust_decimal::Decimal;

use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethod {
    Cash,
    CreditCard,
    BankTransfer,
    Invoice,
}

impl PaymentMethod {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::Cash => "cash",
            Self::CreditCard => "credit_card",
            Self::BankTransfer => "bank_transfer",
            Self::Invoice => "invoice",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "cash" => Some(Self::Cash),
            "credit_card" => Some(Self::CreditCard),
            "bank_transfer" => Some(Self::BankTransfer),
            "invoice" => Some(Self::Invoice),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: Uuid,

    pub folio_id: Uuid,

    pub amount: Decimal,

    pub method: PaymentMethod,

    pub external_reference: Option<String>,

    pub paid_at: DateTime<Utc>,
}

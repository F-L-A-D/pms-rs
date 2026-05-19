use serde::{Deserialize, Serialize};

use chrono::NaiveDate;

use rust_decimal::Decimal;

use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceivableStatus {
    Open,
    Settled,
    Disputed,
    WrittenOff,
    Voided,
}

impl ReceivableStatus {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Settled => "settled",
            Self::Disputed => "disputed",
            Self::WrittenOff => "written_off",
            Self::Voided => "voided",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "open" => Some(Self::Open),
            "settled" => Some(Self::Settled),
            "disputed" => Some(Self::Disputed),
            "written_off" => Some(Self::WrittenOff),
            "voided" => Some(Self::Voided),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Receivable {
    pub id: Uuid,
    pub invoice_id: Uuid,
    pub outstanding_amount: Decimal,
    pub due_date: NaiveDate,
    pub status: ReceivableStatus,
}

impl Receivable {
    pub fn new(
        id: Uuid,
        invoice_id: Uuid,
        outstanding_amount: Decimal,
        due_date: NaiveDate,
    ) -> Result<Self, String> {
        if outstanding_amount < Decimal::ZERO {
            return Err("outstanding amount cannot be negative".into());
        }

        Ok(Self {
            id,
            invoice_id,
            outstanding_amount,
            due_date,
            status: ReceivableStatus::Open,
        })
    }

    pub fn settle(&mut self) -> Result<(), String> {
        if self.outstanding_amount != Decimal::ZERO {
            return Err("cannot settle receivable with outstanding balance".into());
        }

        self.status = ReceivableStatus::Settled;

        Ok(())
    }

    pub fn mark_disputed(&mut self) {
        self.status = ReceivableStatus::Disputed;
    }

    pub fn resolve_dispute(&mut self) {
        self.status = ReceivableStatus::Open;
    }

    pub fn write_off(&mut self) {
        self.outstanding_amount = Decimal::ZERO;
        self.status = ReceivableStatus::WrittenOff;
    }

    pub fn void(&mut self) {
        self.outstanding_amount = Decimal::ZERO;
        self.status = ReceivableStatus::Voided;
    }

    pub fn reopen_with_outstanding_amount(&mut self, outstanding_amount: Decimal) {
        self.outstanding_amount = outstanding_amount;
        self.status = ReceivableStatus::Open;
    }
}

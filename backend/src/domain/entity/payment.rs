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
    Online,
    Other,
}

impl PaymentMethod {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::Cash => "cash",
            Self::CreditCard => "credit_card",
            Self::BankTransfer => "bank_transfer",
            Self::Online => "online",
            Self::Other => "other",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "cash" => Some(Self::Cash),
            "credit_card" => Some(Self::CreditCard),
            "bank_transfer" => Some(Self::BankTransfer),
            "online" => Some(Self::Online),
            "other" => Some(Self::Other),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    Unapplied,
    PartiallyApplied,
    Applied,
    PartiallyRefunded,
    Refunded,
    Voided,
}

impl PaymentStatus {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::Unapplied => "unapplied",
            Self::PartiallyApplied => "partially_applied",
            Self::Applied => "applied",
            Self::PartiallyRefunded => "partially_refunded",
            Self::Refunded => "refunded",
            Self::Voided => "voided",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "unapplied" => Some(Self::Unapplied),
            "partially_applied" => Some(Self::PartiallyApplied),
            "applied" => Some(Self::Applied),
            "partially_refunded" => Some(Self::PartiallyRefunded),
            "refunded" => Some(Self::Refunded),
            "voided" => Some(Self::Voided),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Payment {
    pub id: Uuid,
    pub folio_id: Uuid,
    pub amount: Decimal,
    pub unapplied_amount: Decimal,
    pub refunded_amount: Decimal,
    pub status: PaymentStatus,
    pub method: PaymentMethod,
    pub external_reference: Option<String>,
    pub paid_at: DateTime<Utc>,
}

impl Payment {
    pub fn new(
        id: Uuid,
        folio_id: Uuid,
        amount: Decimal,
        method: PaymentMethod,
        external_reference: Option<String>,
        paid_at: DateTime<Utc>,
    ) -> Result<Self, String> {
        if amount <= Decimal::ZERO {
            return Err("payment amount must be positive".into());
        }

        Ok(Self {
            id,
            folio_id,
            amount,
            unapplied_amount: amount,
            refunded_amount: Decimal::ZERO,
            status: PaymentStatus::Unapplied,
            method,
            external_reference,
            paid_at,
        })
    }

    pub fn apply(&mut self, amount: Decimal) -> Result<(), String> {
        if amount <= Decimal::ZERO {
            return Err("application amount must be positive".into());
        }

        if amount > self.unapplied_amount {
            return Err("application amount exceeds unapplied amount".into());
        }

        self.unapplied_amount -= amount;
        self.refresh_status();

        Ok(())
    }

    pub fn reverse_application(&mut self, amount: Decimal) -> Result<(), String> {
        if amount <= Decimal::ZERO {
            return Err("reverse amount must be positive".into());
        }

        let max_unapplied = self.amount - self.refunded_amount;

        if self.unapplied_amount + amount > max_unapplied {
            return Err("reverse amount exceeds payment balance".into());
        }

        self.unapplied_amount += amount;
        self.refresh_status();

        Ok(())
    }

    pub fn refund(&mut self, amount: Decimal) -> Result<(), String> {
        if amount <= Decimal::ZERO {
            return Err("refund amount must be positive".into());
        }

        if amount > self.unapplied_amount {
            return Err("refund amount exceeds unapplied amount".into());
        }

        self.unapplied_amount -= amount;
        self.refunded_amount += amount;
        self.refresh_status();

        Ok(())
    }

    pub fn void(&mut self) -> Result<(), String> {
        if self.unapplied_amount != self.amount {
            return Err("only fully unapplied payment can be voided".into());
        }

        if self.refunded_amount != Decimal::ZERO {
            return Err("refunded payment cannot be voided".into());
        }

        self.unapplied_amount = Decimal::ZERO;
        self.status = PaymentStatus::Voided;

        Ok(())
    }

    fn refresh_status(&mut self) {
        if self.status == PaymentStatus::Voided {
            return;
        }

        if self.refunded_amount == self.amount {
            self.status = PaymentStatus::Refunded;
        } else if self.refunded_amount > Decimal::ZERO {
            self.status = PaymentStatus::PartiallyRefunded;
        } else if self.unapplied_amount == self.amount {
            self.status = PaymentStatus::Unapplied;
        } else if self.unapplied_amount == Decimal::ZERO {
            self.status = PaymentStatus::Applied;
        } else {
            self.status = PaymentStatus::PartiallyApplied;
        }
    }
}

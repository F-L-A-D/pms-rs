use chrono::{
    DateTime,
    Utc,
};

use rust_decimal::Decimal;

use serde::{
    Deserialize,
    Serialize,
};

use uuid::Uuid;

use crate::domain::entity::payment::PaymentMethod;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DepositStatus {
    Held,
    PartiallyApplied,
    Applied,
    PartiallyRefunded,
    Refunded,
    Forfeited,
    Voided,
}

impl DepositStatus {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::Held => "held",
            Self::PartiallyApplied => "partially_applied",
            Self::Applied => "applied",
            Self::PartiallyRefunded => "partially_refunded",
            Self::Refunded => "refunded",
            Self::Forfeited => "forfeited",
            Self::Voided => "voided",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "held" => Some(Self::Held),
            "partially_applied" => Some(Self::PartiallyApplied),
            "applied" => Some(Self::Applied),
            "partially_refunded" => Some(Self::PartiallyRefunded),
            "refunded" => Some(Self::Refunded),
            "forfeited" => Some(Self::Forfeited),
            "voided" => Some(Self::Voided),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Deposit {
    pub id: Uuid,
    pub folio_id: Uuid,
    pub amount: Decimal,
    pub unapplied_amount: Decimal,
    pub refunded_amount: Decimal,
    pub status: DepositStatus,
    pub method: PaymentMethod,
    pub external_reference: Option<String>,
    pub received_at: DateTime<Utc>,
}

impl Deposit {
    pub fn new(
        id: Uuid,
        folio_id: Uuid,
        amount: Decimal,
        method: PaymentMethod,
        external_reference: Option<String>,
        received_at: DateTime<Utc>,
    ) -> Result<Self, String> {
        if amount <= Decimal::ZERO {
            return Err("deposit amount must be positive".into());
        }

        Ok(Self {
            id,
            folio_id,
            amount,
            unapplied_amount: amount,
            refunded_amount: Decimal::ZERO,
            status: DepositStatus::Held,
            method,
            external_reference,
            received_at,
        })
    }

    pub fn apply(
        &mut self,
        amount: Decimal,
    ) -> Result<(), String> {
        if amount <= Decimal::ZERO {
            return Err("deposit application amount must be positive".into());
        }

        if amount > self.unapplied_amount {
            return Err("deposit application amount exceeds unapplied amount".into());
        }

        self.unapplied_amount -= amount;
        self.refresh_status();

        Ok(())
    }

    pub fn reverse_application(
        &mut self,
        amount: Decimal,
    ) -> Result<(), String> {
        if amount <= Decimal::ZERO {
            return Err("deposit reverse amount must be positive".into());
        }

        let max_unapplied =
            self.amount - self.refunded_amount;

        if self.unapplied_amount + amount > max_unapplied {
            return Err("deposit reverse amount exceeds available balance".into());
        }

        self.unapplied_amount += amount;
        self.refresh_status();

        Ok(())
    }

    pub fn refund(
        &mut self,
        amount: Decimal,
    ) -> Result<(), String> {
        if amount <= Decimal::ZERO {
            return Err("deposit refund amount must be positive".into());
        }

        if amount > self.unapplied_amount {
            return Err("deposit refund amount exceeds unapplied amount".into());
        }

        self.unapplied_amount -= amount;
        self.refunded_amount += amount;
        self.refresh_status();

        Ok(())
    }

    pub fn forfeit(
        &mut self,
        amount: Decimal,
    ) -> Result<(), String> {
        if amount <= Decimal::ZERO {
            return Err("deposit forfeit amount must be positive".into());
        }

        if amount > self.unapplied_amount {
            return Err("deposit forfeit amount exceeds unapplied amount".into());
        }

        self.unapplied_amount -= amount;

        if self.unapplied_amount == Decimal::ZERO {
            self.status = DepositStatus::Forfeited;
        } else {
            self.refresh_status();
        }

        Ok(())
    }

    pub fn void(&mut self) -> Result<(), String> {
        if self.unapplied_amount != self.amount {
            return Err("only fully held deposit can be voided".into());
        }

        if self.refunded_amount != Decimal::ZERO {
            return Err("refunded deposit cannot be voided".into());
        }

        self.unapplied_amount = Decimal::ZERO;
        self.status = DepositStatus::Voided;

        Ok(())
    }

    fn refresh_status(&mut self) {
        if self.status == DepositStatus::Voided {
            return;
        }

        if self.refunded_amount == self.amount {
            self.status = DepositStatus::Refunded;
        } else if self.refunded_amount > Decimal::ZERO {
            self.status = DepositStatus::PartiallyRefunded;
        } else if self.unapplied_amount == self.amount {
            self.status = DepositStatus::Held;
        } else if self.unapplied_amount == Decimal::ZERO {
            self.status = DepositStatus::Applied;
        } else {
            self.status = DepositStatus::PartiallyApplied;
        }
    }
}
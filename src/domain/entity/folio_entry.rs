use chrono::{DateTime, Utc};

use rust_decimal::Decimal;

use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FolioEntryType {
    RoomCharge,
    TaxCharge,

    DepositReceived,

    PaymentApplied,
    RefundApplied,

    RateCorrection,
    TaxCorrection,
    ManualAdjustment,

    Writeoff,

    TransferIn,
    TransferOut,

    InvoiceIssued,
    InvoiceVoided,
}

impl FolioEntryType {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::RoomCharge => "room_charge",
            Self::TaxCharge => "tax_charge",

            Self::DepositReceived => "deposit_received",

            Self::PaymentApplied => "payment_applied",
            Self::RefundApplied => "refund_applied",

            Self::RateCorrection => "rate_correction",
            Self::TaxCorrection => "tax_correction",
            Self::ManualAdjustment => "manual_adjustment",

            Self::Writeoff => "writeoff",

            Self::TransferIn => "transfer_in",
            Self::TransferOut => "transfer_out",

            Self::InvoiceIssued => "invoice_issued",
            Self::InvoiceVoided => "invoice_voided",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "room_charge" => Some(Self::RoomCharge),
            "tax_charge" => Some(Self::TaxCharge),

            "deposit_received" => Some(Self::DepositReceived),

            "payment_applied" => Some(Self::PaymentApplied),
            "refund_applied" => Some(Self::RefundApplied),

            "rate_correction" => Some(Self::RateCorrection),
            "tax_correction" => Some(Self::TaxCorrection),
            "manual_adjustment" => Some(Self::ManualAdjustment),

            "writeoff" => Some(Self::Writeoff),

            "transfer_in" => Some(Self::TransferIn),
            "transfer_out" => Some(Self::TransferOut),

            "invoice_issued" => Some(Self::InvoiceIssued),
            "invoice_voided" => Some(Self::InvoiceVoided),

            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolioEntry {
    pub id: Uuid,

    pub folio_id: Uuid,

    pub entry_type: FolioEntryType,

    pub amount: Decimal,

    pub occurred_at: DateTime<Utc>,

    pub memo: Option<String>,
}

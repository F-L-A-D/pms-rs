use serde::{
    Deserialize,
    Serialize,
};

use chrono::{
    DateTime,
    Utc,
};

use uuid::Uuid;

use crate::domain::{
    folio::{
        Folio,
        FolioStatus,
    },

    folio_entry::{
        FolioEntryType,
        FolioEntry,
    },
};

#[derive(Deserialize)]
pub struct OpenFolioRequest {
    pub reservation_id: Uuid,
}

#[derive(Deserialize)]
pub struct PostRoomChargeRequest {
    pub amount: i64,
    pub description: String,
}

#[derive(Deserialize)]
pub struct PostPaymentRequest {
    pub amount: i64,
    pub description: String,
}

#[derive(Deserialize)]
pub struct AssignBillingAccountRequest {
    pub billing_account_id: String,
}

#[derive(Deserialize)]
pub struct IssueInvoiceRequest {
    pub folio_id: Uuid,
}

#[derive(Serialize)]
pub struct FolioResponse {
    pub folio_id: Uuid,
    pub status: String,
}

#[derive(Serialize)]
pub struct BalanceResponse {
    pub folio_id: Uuid,
    pub balance: i64,
}

#[derive(Serialize)]
pub struct FolioEntryResponse {
    pub id: Uuid,
    pub entry_type: String,
    pub amount: i64,
    pub description: Option<String>,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct IssueInvoiceResponse {
    pub invoice_id: Uuid,
    pub receivable_id: Uuid,
}

impl From<Folio>
    for FolioResponse
{
    fn from(
        folio: Folio,
    ) -> Self {

        Self {

            folio_id:
                folio.id,

            status:
                match folio.status {

                    FolioStatus::Open =>
                        "Open".into(),

                    FolioStatus::Closed =>
                        "Closed".into(),
                },
        }
    }
}

impl From<FolioEntry>
    for FolioEntryResponse
{
    fn from(
        entry: FolioEntry,
    ) -> Self {

        Self {

            id:
                entry.id,

            entry_type:
                match entry.entry_type {

                    FolioEntryType
                        ::RoomCharge =>
                    {
                        "RoomCharge"
                            .into()
                    }

                    FolioEntryType
                        ::Payment =>
                    {
                        "Payment"
                            .into()
                    }

                    FolioEntryType
                        ::Adjustment =>
                    {
                        "Adjustment"
                            .into()
                    }
                },

            amount:
                entry.amount,

            description:
                entry.description,

            occurred_at:
                entry.occurred_at,
        }
    }
}
use rust_decimal::Decimal;

use uuid::Uuid;

use crate::{
    api::dto::billing::response::{
        folio_detail_response::FolioDetailResponse, folio_entry_response::FolioEntryResponse,
        folio_response::FolioResponse,
    },
    db::connection::Db,
    domain::entity::folio_entry::{FolioEntry, FolioEntryType},
    error::app_error::{infra, not_found, AppResult},
    repository::sqlite::operational::billing::{
        folio_entry_repository::SqliteFolioEntryRepository, folio_repository::SqliteFolioRepository,
    },
};

pub async fn execute(db: &Db, folio_id: Uuid) -> AppResult<FolioDetailResponse> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let folio = SqliteFolioRepository::find_by_id(&mut tx, folio_id)
            .await?
            .ok_or_else(|| not_found("folio not found"))?;

        let entries = SqliteFolioEntryRepository::find_by_folio_id(&mut tx, folio_id).await?;

        let mut total_charges = Decimal::ZERO;

        let mut total_payments = Decimal::ZERO;

        for entry in &entries {
            let signed_amount = signed_balance_amount(entry);

            if signed_amount >= Decimal::ZERO {
                total_charges += signed_amount;
            } else {
                total_payments += -signed_amount;
            }
        }

        let balance = total_charges - total_payments;

        Ok(FolioDetailResponse {
            folio: FolioResponse {
                id: folio.id,
                reservation_id: folio.reservation_id,
                billing_account_id: folio.billing_account_id,
                status: folio.status,
                created_at: folio.created_at,
            },

            entries: entries
                .into_iter()
                .map(|entry| FolioEntryResponse {
                    id: entry.id,
                    folio_id: entry.folio_id,
                    entry_type: entry.entry_type,
                    amount: entry.amount,
                    occurred_at: entry.occurred_at,
                    memo: entry.memo,
                })
                .collect(),

            total_charges,
            total_payments,
            balance,
        })
    }
    .await;

    match result {
        Ok(detail) => {
            tx.commit().await.map_err(infra)?;

            Ok(detail)
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}

fn signed_balance_amount(entry: &FolioEntry) -> Decimal {
    match entry.entry_type {
        FolioEntryType::RoomCharge
        | FolioEntryType::TaxCharge
        | FolioEntryType::RateCorrection
        | FolioEntryType::TaxCorrection
        | FolioEntryType::ManualAdjustment
        | FolioEntryType::TransferIn
        | FolioEntryType::InvoiceIssued
        | FolioEntryType::RefundApplied => entry.amount,

        FolioEntryType::DepositReceived
        | FolioEntryType::PaymentApplied
        | FolioEntryType::TransferOut
        | FolioEntryType::Writeoff
        | FolioEntryType::InvoiceVoided => -entry.amount,
    }
}

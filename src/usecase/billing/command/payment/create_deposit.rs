use chrono::Utc;

use uuid::Uuid;

use crate::{
    api::dto::billing::input::create_deposit_input::CreateDepositInput,
    db::connection::Db,
    domain::entity::{
        folio::FolioStatus,
        folio_entry::{FolioEntry, FolioEntryType},
        payment::Payment,
    },
    error::app_error::{domain, infra, not_found, validation, AppResult},
    repository::sqlite::operational::{
        folio_entry_repository::SqliteFolioEntryRepository,
        folio_repository::SqliteFolioRepository, payment_repository::SqlitePaymentRepository,
    },
};

pub async fn execute(db: &Db, input: CreateDepositInput) -> AppResult<Payment> {
    let mut tx = db.begin_tx().await;

    let result = async {
        if input.amount <= rust_decimal::Decimal::ZERO {
            return Err(validation("deposit amount must be positive"));
        }

        let folio = SqliteFolioRepository::find_by_id(&mut tx, input.folio_id)
            .await?
            .ok_or_else(|| not_found("folio not found"))?;

        if !matches!(folio.status, FolioStatus::Open) {
            return Err(domain("folio is not open"));
        }

        let payment = Payment {
            id: Uuid::new_v4(),
            folio_id: input.folio_id,
            amount: input.amount,
            method: input.method,
            external_reference: input.external_reference,
            paid_at: Utc::now(),
        };

        SqlitePaymentRepository::save(&mut tx, &payment).await?;

        let entry = FolioEntry {
            id: Uuid::new_v4(),
            folio_id: input.folio_id,
            entry_type: FolioEntryType::DepositReceived,
            amount: input.amount,
            occurred_at: Utc::now(),
            memo: Some("deposit received".to_string()),
        };

        SqliteFolioEntryRepository::save(&mut tx, &entry).await?;

        Ok(payment)
    }
    .await;

    match result {
        Ok(payment) => {
            tx.commit().await.map_err(infra)?;

            Ok(payment)
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}

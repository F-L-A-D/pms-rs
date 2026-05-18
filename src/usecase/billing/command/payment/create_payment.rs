use chrono::Utc;

use uuid::Uuid;

use crate::{
    api::dto::billing::input::create_payment_input::CreatePaymentInput,
    db::connection::Db,
    domain::entity::{
        folio::FolioStatus,
        folio_entry::{FolioEntry, FolioEntryType},
        payment::Payment,
    },
    error::app_error::{domain, infra, not_found, AppResult},
    repository::sqlite::operational::{
        folio_entry_repository::SqliteFolioEntryRepository,
        folio_repository::SqliteFolioRepository, payment_repository::SqlitePaymentRepository,
    },
};

pub async fn execute(db: &Db, input: CreatePaymentInput) -> AppResult<Payment> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let folio = match SqliteFolioRepository::find_by_id(&mut tx, input.folio_id).await? {
            Some(folio) => folio,

            None => {
                return Err(not_found("folio not found"));
            }
        };

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

            entry_type: FolioEntryType::PaymentApplied,

            amount: input.amount,

            occurred_at: Utc::now(),

            memo: Some("payment applied".to_string()),
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

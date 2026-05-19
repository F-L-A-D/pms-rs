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
    domain::semantic::operation_context::OperationContext,
    error::app_error::{domain, infra, not_found, AppResult},
    repository::sqlite::operational::{
        folio_entry_repository::SqliteFolioEntryRepository,
        folio_repository::SqliteFolioRepository, payment_repository::SqlitePaymentRepository,
    },
    usecase::audit::command::record_audit_log::{record_audit_log, RecordAuditLogInput},
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

        record_audit_log(
            &mut tx,
            &OperationContext::api_system(),
            RecordAuditLogInput {
                aggregate_type: "payment".to_string(),
                aggregate_id: payment.id,
                action: "billing.payment.create".to_string(),
                before_json: None,
                after_json: serde_json::json!({
                    "id": payment.id,
                    "folio_id": payment.folio_id,
                    "amount": payment.amount,
                    "method": payment.method,
                    "external_reference": payment.external_reference,
                    "folio_entry_id": entry.id,
                    "folio_entry_type": entry.entry_type,
                })
                .to_string(),
                changed_fields_json: serde_json::json!([
                    {"field_name": "amount", "before_value": null, "after_value": payment.amount.to_string()},
                    {"field_name": "method", "before_value": null, "after_value": payment.method.to_snake()}
                ])
                .to_string(),
                reason: None,
            },
        )
        .await?;

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

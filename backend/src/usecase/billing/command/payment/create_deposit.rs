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
    domain::semantic::{
        operation_change_event::{ChangedField, OperationChangeEvent, OperationType},
        operation_context::OperationContext,
    },
    error::app_error::{domain, infra, not_found, validation, AppResult},
    repository::sqlite::operational::{
        billing::{
            folio_entry_repository::SqliteFolioEntryRepository,
            folio_repository::SqliteFolioRepository, payment_repository::SqlitePaymentRepository,
        },
        operation::operation_change_event_repository::SqliteOperationChangeEventRepository,
    },
    usecase::audit::command::record_audit_log::{record_audit_log, RecordAuditLogInput},
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

        let context = OperationContext::api_system();

        let after_json = serde_json::json!({
            "id": payment.id,
            "payment_id": payment.id,
            "folio_id": payment.folio_id,
            "amount": payment.amount,
            "method": payment.method,
            "external_reference": payment.external_reference,
            "folio_entry_id": entry.id,
            "folio_entry_type": entry.entry_type,
        })
        .to_string();

        let changed_fields_json = serde_json::to_string(&vec![
            ChangedField::new("amount", None, Some(payment.amount.to_string())),
            ChangedField::new("method", None, Some(payment.method.to_snake().to_string())),
        ])
        .map_err(infra)?;

        let operation_event = OperationChangeEvent {
            id: Uuid::new_v4(),
            operation_id: context.operation_id,
            aggregate_type: "payment".to_string(),
            aggregate_id: payment.id,
            operation_type: OperationType::ReceiveDeposit,
            actor: context.actor,
            actor_id: context.actor_id.clone(),
            source: context.source,
            before_json: None,
            after_json: after_json.clone(),
            changed_fields_json: changed_fields_json.clone(),
            occurred_at: Utc::now(),
        };

        SqliteOperationChangeEventRepository::save(&mut tx, &operation_event).await?;

        record_audit_log(
            &mut tx,
            &context,
            RecordAuditLogInput {
                aggregate_type: "payment".to_string(),
                aggregate_id: payment.id,
                action: "billing.deposit.create".to_string(),
                before_json: None,
                after_json,
                changed_fields_json,
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

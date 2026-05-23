use chrono::Utc;

use uuid::Uuid;

use crate::{
    db::connection::Db,
    domain::{
        entity::deposit::Deposit,
        semantic::{
            operation_change_event::{ChangedField, OperationChangeEvent, OperationType},
            operation_context::OperationContext,
        },
    },
    error::app_error::{conflict, infra, not_found, validation, AppResult},
    repository::sqlite::operational::{
        billing::{
            deposit_repository::SqliteDepositRepository, folio_repository::SqliteFolioRepository,
        },
        operation::operation_change_event_repository::SqliteOperationChangeEventRepository,
    },
    usecase::audit::command::record_audit_log::{record_audit_log, RecordAuditLogInput},
};

use crate::api::dto::billing::input::create_deposit_input::CreateDepositInput;

pub async fn execute(db: &Db, input: CreateDepositInput) -> AppResult<Deposit> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let folio = SqliteFolioRepository::find_by_id(&mut tx, input.folio_id)
            .await?
            .ok_or_else(|| not_found("folio not found"))?;

        let deposit = Deposit::new(
            Uuid::new_v4(),
            folio.id,
            input.amount,
            input.method,
            input.external_reference,
            Utc::now(),
        )
        .map_err(validation)?;

        SqliteDepositRepository::save(&mut tx, &deposit).await?;

        let context = OperationContext::api_system();

        let after_json = serde_json::json!({
            "id": deposit.id,
            "deposit_id": deposit.id,
            "folio_id": deposit.folio_id,
            "amount": deposit.amount,
            "unapplied_amount": deposit.unapplied_amount,
            "refunded_amount": deposit.refunded_amount,
            "status": deposit.status,
            "method": deposit.method,
            "external_reference": deposit.external_reference,
            "received_at": deposit.received_at,
        })
        .to_string();

        let changed_fields_json = serde_json::to_string(&vec![
            ChangedField::new("status", None, Some(deposit.status.to_snake().to_string())),
            ChangedField::new("amount", None, Some(deposit.amount.to_string())),
            ChangedField::new(
                "unapplied_amount",
                None,
                Some(deposit.unapplied_amount.to_string()),
            ),
        ])
        .map_err(infra)?;

        let operation_event = OperationChangeEvent {
            id: Uuid::new_v4(),
            operation_id: context.operation_id,
            aggregate_type: "deposit".to_string(),
            aggregate_id: deposit.id,
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
                aggregate_type: "deposit".to_string(),
                aggregate_id: deposit.id,
                action: "deposit.receive".to_string(),
                before_json: None,
                after_json,
                changed_fields_json,
                reason: input.reason,
            },
        )
        .await?;

        Ok(deposit)
    }
    .await;

    match result {
        Ok(deposit) => {
            tx.commit().await.map_err(infra)?;

            Ok(deposit)
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}

use chrono::Utc;

use uuid::Uuid;

use crate::{
    db::connection::Db,
    domain::{
        entity::receivable::{
            Receivable,
            ReceivableStatus,
        },
        semantic::{
            operation_change_event::{
                ChangedField,
                OperationChangeEvent,
                OperationType,
            },
            operation_context::OperationContext,
            settlement_transition::{
                SettlementTransition,
                SettlementTransitionType,
            },
        },
    },
    error::app_error::{
        conflict,
        infra,
        not_found,
        AppResult,
    },
    repository::sqlite::{
        behavioral::settlement_transition_repository::SqliteSettlementTransitionRepository,
        operational::{
            billing::receivable_repository::SqliteReceivableRepository,
            operation::operation_change_event_repository::SqliteOperationChangeEventRepository,
        },
    },
    usecase::audit::command::record_audit_log::{
        record_audit_log,
        RecordAuditLogInput,
    },
};

pub async fn execute(
    db: &Db,
    receivable_id: Uuid,
    reason: Option<String>,
) -> AppResult<Receivable> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let mut receivable =
            SqliteReceivableRepository::find_by_id(
                &mut tx,
                receivable_id,
            )
            .await?
            .ok_or_else(|| not_found("receivable not found"))?;

        if !matches!(
            receivable.status,
            ReceivableStatus::Open | ReceivableStatus::Disputed
        ) {
            return Err(conflict("receivable cannot be written off"));
        }

        let before_receivable_status =
            receivable.status.clone();

        let before_outstanding_amount =
            receivable.outstanding_amount;

        let written_off_amount =
            receivable.outstanding_amount;

        let before_json =
            serde_json::json!({
                "id": receivable.id,
                "receivable_id": receivable.id,
                "invoice_id": receivable.invoice_id,
                "outstanding_amount": receivable.outstanding_amount,
                "due_date": receivable.due_date,
                "status": receivable.status,
            })
            .to_string();

        receivable.write_off();

        SqliteReceivableRepository::save(
            &mut tx,
            &receivable,
        )
        .await?;

        SqliteSettlementTransitionRepository::save(
            &mut tx,
            &SettlementTransition {
                id: Uuid::new_v4(),
                receivable_id: receivable.id,
                transition_type:
                    SettlementTransitionType::ReceivableWrittenOff,
                amount: written_off_amount,
                occurred_at: Utc::now(),
            },
        )
        .await?;

        let context =
            OperationContext::api_system();

        let after_json =
            serde_json::json!({
                "id": receivable.id,
                "receivable_id": receivable.id,
                "invoice_id": receivable.invoice_id,
                "outstanding_amount": receivable.outstanding_amount,
                "due_date": receivable.due_date,
                "status": receivable.status,
            })
            .to_string();

        let changed_fields_json =
            serde_json::to_string(&vec![
                ChangedField::new(
                    "status",
                    Some(
                        before_receivable_status
                            .to_snake()
                            .to_string(),
                    ),
                    Some(
                        receivable
                            .status
                            .to_snake()
                            .to_string(),
                    ),
                ),
                ChangedField::new(
                    "outstanding_amount",
                    Some(before_outstanding_amount.to_string()),
                    Some(receivable.outstanding_amount.to_string()),
                ),
            ])
            .map_err(infra)?;

        let operation_event =
            OperationChangeEvent {
                id: Uuid::new_v4(),
                operation_id: context.operation_id,
                aggregate_type: "receivable".to_string(),
                aggregate_id: receivable.id,
                operation_type: OperationType::WriteOffReceivable,
                actor: context.actor,
                actor_id: context.actor_id.clone(),
                source: context.source,
                before_json: Some(before_json.clone()),
                after_json: after_json.clone(),
                changed_fields_json:
                    changed_fields_json.clone(),
                occurred_at: Utc::now(),
            };

        SqliteOperationChangeEventRepository::save(
            &mut tx,
            &operation_event,
        )
        .await?;

        record_audit_log(
            &mut tx,
            &context,
            RecordAuditLogInput {
                aggregate_type: "receivable".to_string(),
                aggregate_id: receivable.id,
                action: "receivable.write_off".to_string(),
                before_json: Some(before_json),
                after_json,
                changed_fields_json,
                reason,
            },
        )
        .await?;

        Ok(receivable)
    }
    .await;

    match result {
        Ok(receivable) => {
            tx.commit().await.map_err(infra)?;

            Ok(receivable)
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}
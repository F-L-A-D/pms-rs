use chrono::Utc;

use rust_decimal::Decimal;

use uuid::Uuid;

use crate::{
    api::dto::billing::input::refund_deposit_input::RefundDepositInput,
    db::connection::Db,
    domain::{
        entity::deposit_refund::DepositRefund,
        semantic::{
            operation_change_event::{ChangedField, OperationChangeEvent, OperationType},
            operation_context::OperationContext,
        },
    },
    error::app_error::{conflict, infra, not_found, validation, AppResult},
    repository::sqlite::operational::{
        billing::{
            deposit_refund_repository::SqliteDepositRefundRepository,
            deposit_repository::SqliteDepositRepository,
        },
        operation::operation_change_event_repository::SqliteOperationChangeEventRepository,
    },
    usecase::audit::command::record_audit_log::{record_audit_log, RecordAuditLogInput},
};

pub async fn execute(db: &Db, input: RefundDepositInput) -> AppResult<DepositRefund> {
    let mut tx = db.begin_tx().await;

    let result = async {
        if input.amount <= Decimal::ZERO {
            return Err(validation("deposit refund amount must be positive"));
        }

        let mut deposit = SqliteDepositRepository::find_by_id(&mut tx, input.deposit_id)
            .await?
            .ok_or_else(|| not_found("deposit not found"))?;

        let before_refunded_amount = deposit.refunded_amount;

        let before_unapplied_amount = deposit.unapplied_amount;

        let before_deposit_status = deposit.status;

        let before_json = serde_json::json!({
            "deposit_id": deposit.id,
            "folio_id": deposit.folio_id,
            "amount": deposit.amount,
            "refund_amount": input.amount,
            "refunded_amount": deposit.refunded_amount,
            "unapplied_amount": deposit.unapplied_amount,
            "deposit_status": deposit.status,
        })
        .to_string();

        deposit.refund(input.amount).map_err(conflict)?;

        let now = Utc::now();

        let refund = DepositRefund::new(
            Uuid::new_v4(),
            deposit.id,
            input.amount,
            input.reason.clone(),
            now,
            now,
        )
        .map_err(conflict)?;

        SqliteDepositRepository::save(&mut tx, &deposit).await?;

        SqliteDepositRefundRepository::save(&mut tx, &refund).await?;

        let context = OperationContext::api_system();

        let after_json = serde_json::json!({
            "deposit_refund_id": refund.id,
            "deposit_id": deposit.id,
            "folio_id": deposit.folio_id,
            "amount": deposit.amount,
            "refund_amount": refund.amount,
            "refunded_amount": deposit.refunded_amount,
            "unapplied_amount": deposit.unapplied_amount,
            "deposit_status": deposit.status,
        })
        .to_string();

        let changed_fields_json = serde_json::to_string(&vec![
            ChangedField::new(
                "deposit_refunded_amount",
                Some(before_refunded_amount.to_string()),
                Some(deposit.refunded_amount.to_string()),
            ),
            ChangedField::new(
                "deposit_unapplied_amount",
                Some(before_unapplied_amount.to_string()),
                Some(deposit.unapplied_amount.to_string()),
            ),
            ChangedField::new(
                "deposit_status",
                Some(before_deposit_status.to_snake().to_string()),
                Some(deposit.status.to_snake().to_string()),
            ),
        ])
        .map_err(infra)?;

        let operation_event = OperationChangeEvent {
            id: Uuid::new_v4(),
            operation_id: context.operation_id,
            aggregate_type: "deposit".to_string(),
            aggregate_id: deposit.id,
            operation_type: OperationType::RefundDeposit,
            actor: context.actor,
            actor_id: context.actor_id.clone(),
            source: context.source,
            before_json: Some(before_json.clone()),
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
                action: "deposit.refund".to_string(),
                before_json: Some(before_json),
                after_json,
                changed_fields_json,
                reason: input.reason,
            },
        )
        .await?;

        Ok(refund)
    }
    .await;

    match result {
        Ok(refund) => {
            tx.commit().await.map_err(infra)?;

            Ok(refund)
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}

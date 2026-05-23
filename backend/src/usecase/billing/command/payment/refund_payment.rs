use chrono::Utc;

use rust_decimal::Decimal;

use uuid::Uuid;

use crate::{
    api::dto::billing::input::refund_payment_input::RefundPaymentInput,
    db::connection::Db,
    domain::{
        entity::payment_refund::PaymentRefund,
        semantic::{
            operation_change_event::{
                ChangedField,
                OperationChangeEvent,
                OperationType,
            },
            operation_context::OperationContext,
        },
    },
    error::app_error::{conflict, infra, not_found, validation, AppResult},
    repository::sqlite::operational::{
        billing::{
            payment_refund_repository::SqlitePaymentRefundRepository,
            payment_repository::SqlitePaymentRepository,
        },
        operation::operation_change_event_repository::SqliteOperationChangeEventRepository,
    },
    usecase::audit::command::record_audit_log::{
        record_audit_log,
        RecordAuditLogInput,
    },
};

pub async fn execute(
    db: &Db,
    input: RefundPaymentInput,
) -> AppResult<PaymentRefund> {
    let mut tx = db.begin_tx().await;

    let result = async {
        if input.amount <= Decimal::ZERO {
            return Err(validation("refund amount must be positive"));
        }

        let mut payment =
            SqlitePaymentRepository::find_by_id(
                &mut tx,
                input.payment_id,
            )
            .await?
            .ok_or_else(|| not_found("payment not found"))?;

        let before_refunded_amount =
            payment.refunded_amount;

        let before_unapplied_amount =
            payment.unapplied_amount;

        let before_payment_status =
            payment.status;

        let before_json =
            serde_json::json!({
                "payment_id": payment.id,
                "folio_id": payment.folio_id,
                "amount": payment.amount,
                "refund_amount": input.amount,
                "refunded_amount": payment.refunded_amount,
                "unapplied_amount": payment.unapplied_amount,
                "payment_status": payment.status,
            })
            .to_string();

        payment
            .refund(input.amount)
            .map_err(conflict)?;

        let now = Utc::now();

        let refund =
            PaymentRefund::new(
                Uuid::new_v4(),
                payment.id,
                input.amount,
                input.reason.clone(),
                now,
                now,
            )
            .map_err(conflict)?;

        SqlitePaymentRepository::save(
            &mut tx,
            &payment,
        )
        .await?;

        SqlitePaymentRefundRepository::save(
            &mut tx,
            &refund,
        )
        .await?;

        let context =
            OperationContext::api_system();

        let after_json =
            serde_json::json!({
                "payment_refund_id": refund.id,
                "payment_id": payment.id,
                "folio_id": payment.folio_id,
                "amount": payment.amount,
                "refund_amount": refund.amount,
                "refunded_amount": payment.refunded_amount,
                "unapplied_amount": payment.unapplied_amount,
                "payment_status": payment.status,
            })
            .to_string();

        let changed_fields_json =
            serde_json::to_string(&vec![
                ChangedField::new(
                    "payment_refunded_amount",
                    Some(before_refunded_amount.to_string()),
                    Some(payment.refunded_amount.to_string()),
                ),
                ChangedField::new(
                    "payment_unapplied_amount",
                    Some(before_unapplied_amount.to_string()),
                    Some(payment.unapplied_amount.to_string()),
                ),
                ChangedField::new(
                    "payment_status",
                    Some(before_payment_status.to_snake().to_string()),
                    Some(payment.status.to_snake().to_string()),
                ),
            ])
            .map_err(infra)?;

        let operation_event =
            OperationChangeEvent {
                id: Uuid::new_v4(),
                operation_id: context.operation_id,
                aggregate_type: "payment".to_string(),
                aggregate_id: payment.id,
                operation_type: OperationType::RefundPayment,
                actor: context.actor,
                actor_id: context.actor_id.clone(),
                source: context.source,
                before_json: Some(before_json.clone()),
                after_json: after_json.clone(),
                changed_fields_json: changed_fields_json.clone(),
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
                aggregate_type: "payment".to_string(),
                aggregate_id: payment.id,
                action: "payment.refund".to_string(),
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
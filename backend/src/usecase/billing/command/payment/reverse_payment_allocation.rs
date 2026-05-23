use chrono::Utc;

use uuid::Uuid;

use crate::{
    db::connection::Db,
    domain::{
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
            billing::{
                payment_allocation_repository::SqlitePaymentAllocationRepository,
                payment_repository::SqlitePaymentRepository,
                receivable_repository::SqliteReceivableRepository,
            },
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
    allocation_id: Uuid,
    reason: Option<String>,
) -> AppResult<()> {
    let mut tx =
        db.begin_tx().await;

    let result = async {
        let mut allocation =
            SqlitePaymentAllocationRepository::find_by_id(
                &mut tx,
                allocation_id,
            )
            .await?
            .ok_or_else(|| {
                not_found("payment allocation not found")
            })?;

        if allocation.reversed_at.is_some() {
            return Err(conflict(
                "payment allocation already reversed",
            ));
        }

        let mut receivable =
            SqliteReceivableRepository::find_by_id(
                &mut tx,
                allocation.receivable_id,
            )
            .await?
            .ok_or_else(|| {
                not_found("receivable not found")
            })?;

        let mut payment =
            SqlitePaymentRepository::find_by_id(
                &mut tx,
                allocation.payment_id,
            )
            .await?
            .ok_or_else(|| {
                not_found("payment not found")
            })?;

        let before_reversed_at =
            allocation.reversed_at;

        let before_receivable_status =
            receivable.status;

        let before_outstanding_amount =
            receivable.outstanding_amount;

        let before_payment_unapplied_amount =
            payment.unapplied_amount;

        let before_payment_status =
            payment.status;

        let before_json =
            serde_json::json!({
                "allocation_id": allocation.id,
                "payment_id": allocation.payment_id,
                "receivable_id": allocation.receivable_id,
                "amount": allocation.amount,
                "reversed_at": allocation.reversed_at,
                "receivable_status": receivable.status,
                "receivable_outstanding_amount": receivable.outstanding_amount,
                "payment_unapplied_amount": payment.unapplied_amount,
                "payment_status": payment.status,
            })
            .to_string();

        payment
            .reverse_application(allocation.amount)
            .map_err(conflict)?;

        let _ = allocation.reverse(Utc::now());

        let new_outstanding_amount =
            receivable.outstanding_amount
                + allocation.amount;

        receivable
            .reopen_with_outstanding_amount(
                new_outstanding_amount,
            );

        SqlitePaymentRepository::save(
            &mut tx,
            &payment,
        )
        .await?;

        SqlitePaymentAllocationRepository::mark_reversed(
            &mut tx,
            &allocation,
        )
        .await?;

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
                    SettlementTransitionType::PaymentAllocationReversed,
                amount: allocation.amount,
                occurred_at: Utc::now(),
            },
        )
        .await?;

        let context =
            OperationContext::api_system();

        let after_json =
            serde_json::json!({
                "allocation_id": allocation.id,
                "payment_id": allocation.payment_id,
                "receivable_id": allocation.receivable_id,
                "amount": allocation.amount,
                "reversed_at": allocation.reversed_at,
                "receivable_status": receivable.status,
                "receivable_outstanding_amount": receivable.outstanding_amount,
                "payment_unapplied_amount": payment.unapplied_amount,
                "payment_status": payment.status,
            })
            .to_string();

        let mut changed_fields =
            Vec::new();

        if before_reversed_at
            != allocation.reversed_at
        {
            changed_fields.push(
                ChangedField::new(
                    "reversed_at",
                    before_reversed_at
                        .map(|value| value.to_rfc3339()),
                    allocation
                        .reversed_at
                        .map(|value| value.to_rfc3339()),
                ),
            );
        }

        if before_receivable_status
            != receivable.status
        {
            changed_fields.push(
                ChangedField::new(
                    "receivable_status",
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
            );
        }

        if before_outstanding_amount
            != receivable.outstanding_amount
        {
            changed_fields.push(
                ChangedField::new(
                    "receivable_outstanding_amount",
                    Some(
                        before_outstanding_amount
                            .to_string(),
                    ),
                    Some(
                        receivable
                            .outstanding_amount
                            .to_string(),
                    ),
                ),
            );
        }

        if before_payment_unapplied_amount
            != payment.unapplied_amount
        {
            changed_fields.push(
                ChangedField::new(
                    "payment_unapplied_amount",
                    Some(
                        before_payment_unapplied_amount
                            .to_string(),
                    ),
                    Some(
                        payment
                            .unapplied_amount
                            .to_string(),
                    ),
                ),
            );
        }

        if before_payment_status
            != payment.status
        {
            changed_fields.push(
                ChangedField::new(
                    "payment_status",
                    Some(
                        before_payment_status
                            .to_snake()
                            .to_string(),
                    ),
                    Some(
                        payment
                            .status
                            .to_snake()
                            .to_string(),
                    ),
                ),
            );
        }

        let changed_fields_json =
            serde_json::to_string(
                &changed_fields,
            )
            .map_err(infra)?;

        let operation_event =
            OperationChangeEvent {
                id: Uuid::new_v4(),
                operation_id: context.operation_id,
                aggregate_type:
                    "payment_allocation".to_string(),
                aggregate_id: allocation.id,
                operation_type:
                    OperationType::ReversePaymentAllocation,
                actor: context.actor,
                actor_id: context.actor_id.clone(),
                source: context.source,
                before_json: Some(
                    before_json.clone(),
                ),
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
                aggregate_type:
                    "payment_allocation".to_string(),
                aggregate_id: allocation.id,
                action:
                    "payment_allocation.reverse"
                        .to_string(),
                before_json: Some(before_json),
                after_json,
                changed_fields_json,
                reason,
            },
        )
        .await?;

        Ok(())
    }
    .await;

    match result {
        Ok(()) => {
            tx.commit()
                .await
                .map_err(infra)?;

            Ok(())
        }

        Err(e) => {
            let _ =
                tx.rollback().await;

            Err(e)
        }
    }
}
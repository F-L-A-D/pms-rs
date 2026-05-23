use chrono::Utc;

use uuid::Uuid;

use crate::{
    api::dto::billing::input::allocate_receivable_payment_input::AllocateReceivablePaymentInput,
    db::connection::Db,
    domain::{
        entity::{
            payment::Payment,
            payment_allocation::PaymentAllocation,
            receivable::ReceivableStatus,
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
        validation,
        AppResult,
    },
    repository::sqlite::{
        behavioral::settlement_transition_repository::SqliteSettlementTransitionRepository,
        operational::{
            billing::{
                invoice_repository::SqliteInvoiceRepository,
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

pub struct ReceivablePaymentAllocationResult {
    pub allocation: PaymentAllocation,
    pub remaining_outstanding_amount: rust_decimal::Decimal,
}

pub async fn execute(
    db: &Db,
    input: AllocateReceivablePaymentInput,
) -> AppResult<ReceivablePaymentAllocationResult> {
    let mut tx =
        db.begin_tx().await;

    let result = async {
        if input.amount <= rust_decimal::Decimal::ZERO {
            return Err(validation("payment amount must be positive"));
        }

        let mut receivable =
            SqliteReceivableRepository::find_by_id(
                &mut tx,
                input.receivable_id,
            )
            .await?
            .ok_or_else(|| not_found("receivable not found"))?;

        if receivable.status != ReceivableStatus::Open {
            return Err(conflict("receivable is not open"));
        }

        if input.amount > receivable.outstanding_amount {
            return Err(conflict(
                "payment amount exceeds outstanding receivable amount",
            ));
        }

        let invoice =
            SqliteInvoiceRepository::find_by_id(
                &mut tx,
                receivable.invoice_id,
            )
            .await?
            .ok_or_else(|| not_found("invoice not found"))?;

        let before_receivable_status =
            receivable.status.clone();

        let before_outstanding_amount =
            receivable.outstanding_amount;

        let before_json =
            serde_json::json!({
                "receivable_id": receivable.id,
                "invoice_id": receivable.invoice_id,
                "folio_id": invoice.folio_id,
                "payment_id": null,
                "allocation_id": null,
                "payment_amount": input.amount,
                "receivable_status": receivable.status,
                "receivable_outstanding_amount": receivable.outstanding_amount,
            })
            .to_string();

        let mut payment =
            Payment::new(
                Uuid::new_v4(),
                invoice.folio_id,
                input.amount,
                input.method,
                input.external_reference,
                Utc::now(),
            )
            .map_err(conflict)?;
        
        payment
            .apply(input.amount)
            .map_err(conflict)?;

        SqlitePaymentRepository::save(
            &mut tx,
            &payment,
        )
        .await?;

        let allocation =
            PaymentAllocation {
                id: Uuid::new_v4(),
                payment_id: payment.id,
                receivable_id: receivable.id,
                amount: input.amount,
                allocated_at: Utc::now(),
                reversed_at: None,
            };

        SqlitePaymentAllocationRepository::save(
            &mut tx,
            &allocation,
        )
        .await?;

        receivable.outstanding_amount -= input.amount;

        SqliteSettlementTransitionRepository::save(
            &mut tx,
            &SettlementTransition {
                id: Uuid::new_v4(),
                receivable_id: receivable.id,
                transition_type:
                    SettlementTransitionType::PaymentAllocated,
                amount: input.amount,
                occurred_at: Utc::now(),
            },
        )
        .await?;

        if receivable.outstanding_amount == rust_decimal::Decimal::ZERO {
            receivable.settle().map_err(validation)?;

            SqliteSettlementTransitionRepository::save(
                &mut tx,
                &SettlementTransition {
                    id: Uuid::new_v4(),
                    receivable_id: receivable.id,
                    transition_type:
                        SettlementTransitionType::ReceivableSettled,
                    amount: input.amount,
                    occurred_at: Utc::now(),
                },
            )
            .await?;
        }

        SqliteReceivableRepository::save(
            &mut tx,
            &receivable,
        )
        .await?;

        let context =
            OperationContext::api_system();

        let after_json =
            serde_json::json!({
                "receivable_id": receivable.id,
                "invoice_id": receivable.invoice_id,
                "folio_id": invoice.folio_id,
                "payment_id": payment.id,
                "allocation_id": allocation.id,
                "payment_amount": payment.amount,
                "payment_method": payment.method,
                "payment_reference": payment.external_reference,
                "receivable_status": receivable.status,
                "receivable_outstanding_amount": receivable.outstanding_amount,
            })
            .to_string();

        let mut changed_fields =
            vec![];

        if before_outstanding_amount
            != receivable.outstanding_amount
        {
            changed_fields.push(
                ChangedField::new(
                    "receivable_outstanding_amount",
                    Some(before_outstanding_amount.to_string()),
                    Some(receivable.outstanding_amount.to_string()),
                ),
            );
        }

        if before_receivable_status
            != receivable.status
        {
            changed_fields.push(
                ChangedField::new(
                    "receivable_status",
                    Some(before_receivable_status.to_snake().to_string()),
                    Some(receivable.status.to_snake().to_string()),
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
                aggregate_type: "payment_allocation".to_string(),
                aggregate_id: allocation.id,
                operation_type:
                    OperationType::AllocateReceivablePayment,
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
                aggregate_type: "payment_allocation".to_string(),
                aggregate_id: allocation.id,
                action: "receivable.payment.allocate".to_string(),
                before_json: Some(before_json),
                after_json,
                changed_fields_json,
                reason: input.reason,
            },
        )
        .await?;

        Ok(ReceivablePaymentAllocationResult {
            allocation,
            remaining_outstanding_amount:
                receivable.outstanding_amount,
        })
    }
    .await;

    match result {
        Ok(result) => {
            tx.commit().await.map_err(infra)?;

            Ok(result)
        }

        Err(e) => {
            let _ =
                tx.rollback().await;

            Err(e)
        }
    }
}
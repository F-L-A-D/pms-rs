use chrono::Utc;

use uuid::Uuid;

use crate::{
    api::dto::billing::input::allocate_existing_payment_input::AllocateExistingPaymentInput,
    db::connection::Db,
    domain::{
        entity::{payment_allocation::PaymentAllocation, receivable::ReceivableStatus},
        semantic::{
            operation_change_event::{ChangedField, OperationChangeEvent, OperationType},
            operation_context::OperationContext,
            settlement_transition::{SettlementTransition, SettlementTransitionType},
        },
    },
    error::app_error::{conflict, infra, not_found, validation, AppResult},
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
    usecase::audit::command::record_audit_log::{record_audit_log, RecordAuditLogInput},
};

pub struct ExistingPaymentAllocationResult {
    pub allocation: PaymentAllocation,
    pub remaining_outstanding_amount: rust_decimal::Decimal,
}

pub async fn execute(
    db: &Db,
    input: AllocateExistingPaymentInput,
) -> AppResult<ExistingPaymentAllocationResult> {
    let mut tx = db.begin_tx().await;

    let result = async {
        if input.amount <= rust_decimal::Decimal::ZERO {
            return Err(validation("allocation amount must be positive"));
        }

        let mut payment = SqlitePaymentRepository::find_by_id(&mut tx, input.payment_id)
            .await?
            .ok_or_else(|| not_found("payment not found"))?;

        let mut receivable = SqliteReceivableRepository::find_by_id(&mut tx, input.receivable_id)
            .await?
            .ok_or_else(|| not_found("receivable not found"))?;

        let invoice = SqliteInvoiceRepository::find_by_id(&mut tx, receivable.invoice_id)
            .await?
            .ok_or_else(|| not_found("invoice not found"))?;

        if receivable.status != ReceivableStatus::Open {
            return Err(conflict("receivable is not open"));
        }

        if payment.folio_id != invoice.folio_id {
            return Err(conflict(
                "payment and receivable belong to different folios",
            ));
        }

        if input.amount > payment.unapplied_amount {
            return Err(conflict(
                "allocation amount exceeds payment unapplied amount",
            ));
        }

        if input.amount > receivable.outstanding_amount {
            return Err(conflict(
                "allocation amount exceeds outstanding receivable amount",
            ));
        }

        let before_payment_unapplied_amount = payment.unapplied_amount;

        let before_payment_status = payment.status;

        let before_receivable_status = receivable.status.clone();

        let before_outstanding_amount = receivable.outstanding_amount;

        let before_json = serde_json::json!({
            "payment_id": payment.id,
            "receivable_id": receivable.id,
            "folio_id": invoice.folio_id,
            "allocation_id": null,
            "allocation_amount": input.amount,
            "payment_unapplied_amount": payment.unapplied_amount,
            "payment_status": payment.status,
            "receivable_status": receivable.status,
            "receivable_outstanding_amount": receivable.outstanding_amount,
        })
        .to_string();

        payment.apply(input.amount).map_err(conflict)?;

        let allocation = PaymentAllocation {
            id: Uuid::new_v4(),
            payment_id: payment.id,
            receivable_id: receivable.id,
            amount: input.amount,
            allocated_at: Utc::now(),
            reversed_at: None,
        };

        SqlitePaymentAllocationRepository::save(&mut tx, &allocation).await?;

        receivable.outstanding_amount -= input.amount;

        SqliteSettlementTransitionRepository::save(
            &mut tx,
            &SettlementTransition {
                id: Uuid::new_v4(),
                receivable_id: receivable.id,
                transition_type: SettlementTransitionType::PaymentAllocated,
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
                    transition_type: SettlementTransitionType::ReceivableSettled,
                    amount: input.amount,
                    occurred_at: Utc::now(),
                },
            )
            .await?;
        }

        SqlitePaymentRepository::save(&mut tx, &payment).await?;

        SqliteReceivableRepository::save(&mut tx, &receivable).await?;

        let context = OperationContext::api_system();

        let after_json = serde_json::json!({
            "payment_id": payment.id,
            "receivable_id": receivable.id,
            "folio_id": invoice.folio_id,
            "allocation_id": allocation.id,
            "allocation_amount": allocation.amount,
            "payment_unapplied_amount": payment.unapplied_amount,
            "payment_status": payment.status,
            "receivable_status": receivable.status,
            "receivable_outstanding_amount": receivable.outstanding_amount,
        })
        .to_string();

        let changed_fields_json = serde_json::to_string(&vec![
            ChangedField::new(
                "payment_unapplied_amount",
                Some(before_payment_unapplied_amount.to_string()),
                Some(payment.unapplied_amount.to_string()),
            ),
            ChangedField::new(
                "payment_status",
                Some(before_payment_status.to_snake().to_string()),
                Some(payment.status.to_snake().to_string()),
            ),
            ChangedField::new(
                "receivable_outstanding_amount",
                Some(before_outstanding_amount.to_string()),
                Some(receivable.outstanding_amount.to_string()),
            ),
            ChangedField::new(
                "receivable_status",
                Some(before_receivable_status.to_snake().to_string()),
                Some(receivable.status.to_snake().to_string()),
            ),
        ])
        .map_err(infra)?;

        let operation_event = OperationChangeEvent {
            id: Uuid::new_v4(),
            operation_id: context.operation_id,
            aggregate_type: "payment_allocation".to_string(),
            aggregate_id: allocation.id,
            operation_type: OperationType::AllocateExistingPayment,
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
                aggregate_type: "payment_allocation".to_string(),
                aggregate_id: allocation.id,
                action: "payment.allocate".to_string(),
                before_json: Some(before_json),
                after_json,
                changed_fields_json,
                reason: input.reason,
            },
        )
        .await?;

        Ok(ExistingPaymentAllocationResult {
            allocation,
            remaining_outstanding_amount: receivable.outstanding_amount,
        })
    }
    .await;

    match result {
        Ok(result) => {
            tx.commit().await.map_err(infra)?;
            Ok(result)
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}

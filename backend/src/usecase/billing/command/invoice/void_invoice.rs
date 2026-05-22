use chrono::Utc;

use uuid::Uuid;

use crate::{
    db::connection::Db,
    domain::{
        entity::{
            invoice::{Invoice, InvoiceStatus},
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
    error::app_error::{conflict, infra, not_found, AppResult},
    repository::sqlite::{
        behavioral::settlement_transition_repository::SqliteSettlementTransitionRepository,
        operational::{
            billing::{
                invoice_repository::SqliteInvoiceRepository,
                payment_allocation_repository::SqlitePaymentAllocationRepository,
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
    invoice_id: Uuid,
    reason: Option<String>,
) -> AppResult<Invoice> {

    let mut tx = db.begin_tx().await;

    let result = async {
        let mut invoice =
            SqliteInvoiceRepository::find_by_id(&mut tx, invoice_id)
                .await?
                .ok_or_else(|| not_found("invoice not found"))?;

        if invoice.status != InvoiceStatus::Issued {
            return Err(conflict("only issued invoices can be voided"));
        }

        let mut receivable =
            SqliteReceivableRepository::find_by_invoice_id(
                &mut tx,
                invoice.id,
            )
            .await?
            .ok_or_else(|| not_found("receivable not found"))?;

        if !matches!(
            receivable.status,
            ReceivableStatus::Open | ReceivableStatus::Disputed
        ) {
            return Err(conflict("invoice receivable cannot be voided"));
        }

        let active_allocations =
            SqlitePaymentAllocationRepository::list_by_receivable_id(
                &mut tx,
                receivable.id,
            )
            .await?
            .into_iter()
            .filter(|allocation| allocation.reversed_at.is_none())
            .collect::<Vec<_>>();

        if !active_allocations.is_empty() {
            return Err(conflict(
                "invoice with active payment allocations cannot be voided",
            ));
        }

        let before_invoice_status =
            invoice.status.clone();

        let before_receivable_status =
            receivable.status.clone();

        let before_json =
            serde_json::json!({
                "id": invoice.id,
                "invoice_id": invoice.id,
                "folio_id": invoice.folio_id,
                "billing_account_id": invoice.billing_account_id,
                "invoice_number": invoice.invoice_number,
                "issued_amount": invoice.issued_amount,
                "due_date": invoice.due_date,
                "status": invoice.status,
                "receivable_id": receivable.id,
                "receivable_status": receivable.status,
            })
            .to_string();

        invoice.void();
        receivable.void();

        SqliteInvoiceRepository::save(
            &mut tx,
            &invoice,
        )
        .await?;

        SqliteReceivableRepository::save(
            &mut tx,
            &receivable,
        )
        .await?;

        let invoice_voided_transition =
            SettlementTransition {
                id: Uuid::new_v4(),
                receivable_id: receivable.id,
                transition_type:
                    SettlementTransitionType::InvoiceVoided,
                amount: invoice.issued_amount,
                occurred_at: Utc::now(),
            };

        SqliteSettlementTransitionRepository::save(
            &mut tx,
            &invoice_voided_transition,
        )
        .await?;

        let receivable_voided_transition =
            SettlementTransition {
                id: Uuid::new_v4(),
                receivable_id: receivable.id,
                transition_type:
                    SettlementTransitionType::ReceivableVoided,
                amount: invoice.issued_amount,
                occurred_at: Utc::now(),
            };

        SqliteSettlementTransitionRepository::save(
            &mut tx,
            &receivable_voided_transition,
        )
        .await?;

        let context =
            OperationContext::api_system();

        let after_json =
            serde_json::json!({
                "id": invoice.id,
                "invoice_id": invoice.id,
                "folio_id": invoice.folio_id,
                "billing_account_id": invoice.billing_account_id,
                "invoice_number": invoice.invoice_number,
                "issued_amount": invoice.issued_amount,
                "due_date": invoice.due_date,
                "status": invoice.status,
                "receivable_id": receivable.id,
                "receivable_status": receivable.status,
            })
            .to_string();

        let changed_fields_json =
            serde_json::to_string(&vec![
                ChangedField::new(
                    "status",
                    Some(
                        before_invoice_status
                            .to_snake()
                            .to_string(),
                    ),
                    Some(
                        invoice
                            .status
                            .to_snake()
                            .to_string(),
                    ),
                ),
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
            ])
            .map_err(infra)?;

        let operation_event =
            OperationChangeEvent {
                id: Uuid::new_v4(),
                operation_id: context.operation_id,
                aggregate_type: "invoice".to_string(),
                aggregate_id: invoice.id,
                operation_type: OperationType::VoidInvoice,
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
                aggregate_type: "invoice".to_string(),
                aggregate_id: invoice.id,
                action: "invoice.void".to_string(),
                before_json: Some(before_json),
                after_json,
                changed_fields_json,
                reason,
            },
        )
        .await?;

        Ok(invoice)
    }
    .await;

    match result {
        Ok(invoice) => {
            tx.commit().await.map_err(infra)?;

            Ok(invoice)
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}
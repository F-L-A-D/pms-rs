use chrono::Utc;

use uuid::Uuid;

use crate::{
    api::dto::billing::input::create_invoice_input::CreateInvoiceInput,
    db::connection::Db,
    domain::entity::{
        folio::FolioStatus,
        invoice::{Invoice, InvoiceStatus},
        receivable::Receivable,
    },
    domain::semantic::{
        operation_context::OperationContext,
        settlement_transition::{SettlementTransition, SettlementTransitionType},
    },
    error::app_error::{conflict, infra, not_found, validation, AppResult},
    repository::sqlite::{
        behavioral::settlement_transition_repository::SqliteSettlementTransitionRepository,
        operational::billing::{
            billing_account_repository::SqliteBillingAccountRepository,
            folio_repository::SqliteFolioRepository, invoice_repository::SqliteInvoiceRepository,
            receivable_repository::SqliteReceivableRepository,
        },
    },
    usecase::audit::command::record_audit_log::{record_audit_log, RecordAuditLogInput},
};

pub async fn execute(db: &Db, input: CreateInvoiceInput) -> AppResult<Invoice> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let folio = match SqliteFolioRepository::find_by_id(&mut tx, input.folio_id).await? {
            Some(folio) => folio,

            None => {
                return Err(not_found("folio not found"));
            }
        };

        if !matches!(folio.status, FolioStatus::Closed) {
            return Err(conflict("cannot issue invoice unless folio is closed"));
        }

        let billing_account_id = folio
            .billing_account_id
            .ok_or_else(|| conflict("billing account not assigned"))?;

        SqliteBillingAccountRepository::find_by_id(&mut tx, billing_account_id)
            .await?
            .ok_or_else(|| not_found("billing account not found"))?;

        if input.invoice_number.trim().is_empty() {
            return Err(validation("invoice number must not be empty"));
        }

        if input.issued_amount <= rust_decimal::Decimal::ZERO {
            return Err(validation("issued amount must be positive"));
        }

        if SqliteInvoiceRepository::find_by_folio_id(&mut tx, folio.id)
            .await?
            .is_some()
        {
            return Err(conflict("invoice already exists for folio"));
        }

        if SqliteInvoiceRepository::find_by_invoice_number(&mut tx, &input.invoice_number)
            .await?
            .is_some()
        {
            return Err(conflict("invoice number already exists"));
        }

        let invoice = Invoice {
            id: Uuid::new_v4(),

            folio_id: input.folio_id,

            billing_account_id,

            invoice_number: input.invoice_number,

            issued_amount: input.issued_amount,

            due_date: input.due_date,

            status: InvoiceStatus::Issued,

            issued_at: Utc::now(),
        };

        SqliteInvoiceRepository::save(&mut tx, &invoice).await?;

        let receivable = Receivable::new(
            Uuid::new_v4(),
            invoice.id,
            invoice.issued_amount,
            invoice.due_date,
        )
        .map_err(validation)?;

        SqliteReceivableRepository::save(&mut tx, &receivable).await?;

        let invoice_issued_transition = SettlementTransition {
            id: Uuid::new_v4(),
            receivable_id: receivable.id,
            transition_type: SettlementTransitionType::InvoiceIssued,
            amount: invoice.issued_amount,
            occurred_at: Utc::now(),
        };

        SqliteSettlementTransitionRepository::save(&mut tx, &invoice_issued_transition).await?;

        let receivable_opened_transition = SettlementTransition {
            id: Uuid::new_v4(),
            receivable_id: receivable.id,
            transition_type: SettlementTransitionType::ReceivableOpened,
            amount: invoice.issued_amount,
            occurred_at: Utc::now(),
        };

        SqliteSettlementTransitionRepository::save(&mut tx, &receivable_opened_transition).await?;

        record_audit_log(
            &mut tx,
            &OperationContext::api_system(),
            RecordAuditLogInput {
                aggregate_type: "invoice".to_string(),
                aggregate_id: invoice.id,
                action: "invoice.issue".to_string(),
                before_json: None,
                after_json: serde_json::json!({
                    "id": invoice.id,
                    "folio_id": invoice.folio_id,
                    "billing_account_id": invoice.billing_account_id,
                    "invoice_number": invoice.invoice_number,
                    "issued_amount": invoice.issued_amount,
                    "due_date": invoice.due_date,
                    "status": invoice.status,
                    "receivable_id": receivable.id,
                })
                .to_string(),
                changed_fields_json: serde_json::json!([
                    {"field_name": "status", "before_value": null, "after_value": invoice.status.to_snake()},
                    {"field_name": "issued_amount", "before_value": null, "after_value": invoice.issued_amount.to_string()}
                ])
                .to_string(),
                reason: None,
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

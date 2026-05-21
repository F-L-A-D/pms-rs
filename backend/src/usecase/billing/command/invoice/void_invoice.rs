use chrono::Utc;

use uuid::Uuid;

use crate::{
    db::connection::Db,
    domain::{
        entity::{
            invoice::{Invoice, InvoiceStatus},
            receivable::ReceivableStatus,
        },
        semantic::settlement_transition::{SettlementTransition, SettlementTransitionType},
    },
    error::app_error::{conflict, infra, not_found, AppResult},
    repository::sqlite::{
        behavioral::settlement_transition_repository::SqliteSettlementTransitionRepository,
        operational::billing::{
            invoice_repository::SqliteInvoiceRepository,
            payment_allocation_repository::SqlitePaymentAllocationRepository,
            receivable_repository::SqliteReceivableRepository,
        },
    },
};

pub async fn execute(db: &Db, invoice_id: Uuid) -> AppResult<Invoice> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let mut invoice = SqliteInvoiceRepository::find_by_id(&mut tx, invoice_id)
            .await?
            .ok_or_else(|| not_found("invoice not found"))?;

        if invoice.status != InvoiceStatus::Issued {
            return Err(conflict("only issued invoices can be voided"));
        }

        let mut receivable = SqliteReceivableRepository::find_by_invoice_id(&mut tx, invoice.id)
            .await?
            .ok_or_else(|| not_found("receivable not found"))?;

        if !matches!(
            receivable.status,
            ReceivableStatus::Open | ReceivableStatus::Disputed
        ) {
            return Err(conflict("invoice receivable cannot be voided"));
        }

        let active_allocations =
            SqlitePaymentAllocationRepository::list_by_receivable_id(&mut tx, receivable.id)
                .await?
                .into_iter()
                .filter(|allocation| allocation.reversed_at.is_none())
                .collect::<Vec<_>>();

        if !active_allocations.is_empty() {
            return Err(conflict(
                "invoice with active payment allocations cannot be voided",
            ));
        }

        invoice.void();
        receivable.void();

        SqliteInvoiceRepository::save(&mut tx, &invoice).await?;
        SqliteReceivableRepository::save(&mut tx, &receivable).await?;

        SqliteSettlementTransitionRepository::save(
            &mut tx,
            &SettlementTransition {
                id: Uuid::new_v4(),
                receivable_id: receivable.id,
                transition_type: SettlementTransitionType::InvoiceVoided,
                amount: invoice.issued_amount,
                occurred_at: Utc::now(),
            },
        )
        .await?;

        SqliteSettlementTransitionRepository::save(
            &mut tx,
            &SettlementTransition {
                id: Uuid::new_v4(),
                receivable_id: receivable.id,
                transition_type: SettlementTransitionType::ReceivableVoided,
                amount: invoice.issued_amount,
                occurred_at: Utc::now(),
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

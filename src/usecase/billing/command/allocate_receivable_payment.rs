use chrono::Utc;

use uuid::Uuid;

use crate::{
    api::dto::billing::input::allocate_receivable_payment_input::AllocateReceivablePaymentInput,
    db::connection::Db,
    domain::{
        entity::{
            payment::Payment, payment_allocation::PaymentAllocation, receivable::ReceivableStatus,
        },
        semantic::settlement_transition::{SettlementTransition, SettlementTransitionType},
    },
    error::app_error::{conflict, infra, not_found, validation, AppResult},
    repository::sqlite::{
        behavioral::settlement_transition_repository::SqliteSettlementTransitionRepository,
        operational::{
            invoice_repository::SqliteInvoiceRepository,
            payment_allocation_repository::SqlitePaymentAllocationRepository,
            payment_repository::SqlitePaymentRepository,
            receivable_repository::SqliteReceivableRepository,
        },
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
    let mut tx = db.begin_tx().await;

    let result = async {
        if input.amount <= rust_decimal::Decimal::ZERO {
            return Err(validation("payment amount must be positive"));
        }

        let mut receivable = SqliteReceivableRepository::find_by_id(&mut tx, input.receivable_id)
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

        let invoice = SqliteInvoiceRepository::find_by_id(&mut tx, receivable.invoice_id)
            .await?
            .ok_or_else(|| not_found("invoice not found"))?;

        let payment = Payment {
            id: Uuid::new_v4(),
            folio_id: invoice.folio_id,
            amount: input.amount,
            method: input.method,
            external_reference: input.external_reference,
            paid_at: Utc::now(),
        };

        SqlitePaymentRepository::save(&mut tx, &payment).await?;

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

        SqliteReceivableRepository::save(&mut tx, &receivable).await?;

        Ok(ReceivablePaymentAllocationResult {
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

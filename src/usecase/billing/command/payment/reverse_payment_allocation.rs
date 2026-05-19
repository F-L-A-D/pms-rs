use chrono::Utc;

use uuid::Uuid;

use crate::{
    db::connection::Db,
    domain::{
        entity::receivable::ReceivableStatus,
        semantic::settlement_transition::{SettlementTransition, SettlementTransitionType},
    },
    error::app_error::{conflict, infra, not_found, validation, AppResult},
    repository::sqlite::{
        behavioral::settlement_transition_repository::SqliteSettlementTransitionRepository,
        operational::{
            payment_allocation_repository::SqlitePaymentAllocationRepository,
            receivable_repository::SqliteReceivableRepository,
        },
    },
};

pub async fn execute(db: &Db, allocation_id: Uuid) -> AppResult<()> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let mut allocation = SqlitePaymentAllocationRepository::find_by_id(&mut tx, allocation_id)
            .await?
            .ok_or_else(|| not_found("payment allocation not found"))?;

        allocation.reverse(Utc::now()).map_err(validation)?;

        let mut receivable =
            SqliteReceivableRepository::find_by_id(&mut tx, allocation.receivable_id)
                .await?
                .ok_or_else(|| not_found("receivable not found"))?;

        if matches!(
            receivable.status,
            ReceivableStatus::WrittenOff | ReceivableStatus::Voided
        ) {
            return Err(conflict(
                "payment allocation cannot be reversed for closed receivable",
            ));
        }

        let new_outstanding = receivable.outstanding_amount + allocation.amount;

        receivable.reopen_with_outstanding_amount(new_outstanding);

        SqlitePaymentAllocationRepository::mark_reversed(&mut tx, &allocation).await?;
        SqliteReceivableRepository::save(&mut tx, &receivable).await?;

        SqliteSettlementTransitionRepository::save(
            &mut tx,
            &SettlementTransition {
                id: Uuid::new_v4(),
                receivable_id: receivable.id,
                transition_type: SettlementTransitionType::PaymentAllocationReversed,
                amount: allocation.amount,
                occurred_at: Utc::now(),
            },
        )
        .await?;

        Ok(())
    }
    .await;

    match result {
        Ok(()) => {
            tx.commit().await.map_err(infra)?;

            Ok(())
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}

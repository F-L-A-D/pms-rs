use chrono::Utc;

use uuid::Uuid;

use crate::{
    db::connection::Db,
    domain::{
        entity::receivable::{Receivable, ReceivableStatus},
        semantic::settlement_transition::{SettlementTransition, SettlementTransitionType},
    },
    error::app_error::{conflict, infra, not_found, AppResult},
    repository::sqlite::{
        behavioral::settlement_transition_repository::SqliteSettlementTransitionRepository,
        operational::billing::receivable_repository::SqliteReceivableRepository,
    },
};

pub async fn execute(db: &Db, receivable_id: Uuid) -> AppResult<Receivable> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let mut receivable = SqliteReceivableRepository::find_by_id(&mut tx, receivable_id)
            .await?
            .ok_or_else(|| not_found("receivable not found"))?;

        if !matches!(
            receivable.status,
            ReceivableStatus::Open | ReceivableStatus::Disputed
        ) {
            return Err(conflict("receivable cannot be written off"));
        }

        let written_off_amount = receivable.outstanding_amount;

        receivable.write_off();

        SqliteReceivableRepository::save(&mut tx, &receivable).await?;

        SqliteSettlementTransitionRepository::save(
            &mut tx,
            &SettlementTransition {
                id: Uuid::new_v4(),
                receivable_id: receivable.id,
                transition_type: SettlementTransitionType::ReceivableWrittenOff,
                amount: written_off_amount,
                occurred_at: Utc::now(),
            },
        )
        .await?;

        Ok(receivable)
    }
    .await;

    match result {
        Ok(receivable) => {
            tx.commit().await.map_err(infra)?;

            Ok(receivable)
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}

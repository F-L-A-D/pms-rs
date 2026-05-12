use uuid::Uuid;

use crate::{
    db::connection::Db,

    domain::settlement_transition::
        SettlementTransitionType,

    error::app_error::AppResult,

    repository::sqlite::behavioral::
        settlement_transition_repository::
            SqliteSettlementTransitionRepository,
};

pub async fn derive_receivable_balance(
    db: &Db,
    receivable_id: Uuid,
) -> AppResult<i64> {

    let mut tx =
        db.begin_tx().await;

    let transitions =
        SqliteSettlementTransitionRepository
            ::find_by_receivable_id(
                &mut tx,
                receivable_id,
            )
            .await?;

    let mut balance =
        0_i64;

    for transition in transitions {

        match transition.transition_type {

            SettlementTransitionType::
                InvoiceIssued => {

                balance +=
                    transition.amount;
            }

            SettlementTransitionType::
                ReceivableOpened => {
                // lifecycle-only transition
            }
        }
    }

    let _ =
        tx.rollback().await;

    Ok(balance)
}
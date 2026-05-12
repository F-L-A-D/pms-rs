use sqlx::{
    Sqlite,
    Transaction,
};

use uuid::Uuid;

use crate::{
    db::connection::Db,

    error::app_error::{
        AppResult,
    },

    repository::sqlite::operational::
        folio_entry_repository::
            SqliteFolioEntryRepository,
};

pub async fn calculate_balance_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    folio_id: Uuid,
) -> AppResult<i64> {

    let entries =
        SqliteFolioEntryRepository
            ::find_by_folio_id(
                tx,
                folio_id,
            )
            .await?;

    let balance =
        entries
            .iter()
            .map(
                |entry| entry.amount
            )
            .sum();

    Ok(balance)
}

pub async fn calculate_balance(
    db: &Db,
    folio_id: Uuid,
) -> AppResult<i64> {

    let mut tx =
        db.begin_tx().await;

    let balance =
        calculate_balance_in_tx(
            &mut tx,
            folio_id,
        )
        .await?;

    let _ =
        tx.rollback().await;

    Ok(balance)
}
use sqlx::{
    Sqlite,
    Transaction,
};

use uuid::Uuid;

use crate::{
    db::connection::Db,

    error::app_error::AppError,

    repository::sqlite::operational::
        folio_entry_repository::
            SqliteFolioEntryRepository,
};

pub async fn calculate_balance_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    folio_id: Uuid,
) -> Result<i64, String> {

    let entries =
        SqliteFolioEntryRepository::find_by_folio_id(
            tx,
            folio_id,
        )
        .await?;

    let balance =
        entries
            .iter()
            .map(|entry| entry.amount)
            .sum();

    Ok(balance)
}

pub async fn calculate_balance(
    db: &Db,
    folio_id: Uuid,
) -> Result<i64, AppError> {

    let mut tx =
        db.begin_tx().await;

    let balance =
        calculate_balance_in_tx(
            &mut tx,
            folio_id,
        )
        .await
        .map_err(AppError::Validation)?;

    tx.rollback()
        .await
        .map_err(|e| {
            AppError::Validation(
                e.to_string()
            )
        })?;

    Ok(balance)
}
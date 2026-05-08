use crate::db::connection::Db;

use crate::error::app_error::{
    AppError,
    AppResult,
};

use crate::repository::sqlite::operational::
    folio_entry_repository::SqliteFolioEntryRepository;

pub async fn calculate_balance(
    db: &Db,
    folio_id: &str,
) -> AppResult<i64> {

    let mut tx =
        db.begin_tx().await;

    let result = async {

        let entries =
            SqliteFolioEntryRepository::find_by_folio_id(
                &mut tx,
                folio_id,
            )
            .await
            .map_err(AppError::Infrastructure)?;

        let balance =
            entries
                .iter()
                .map(|e| e.amount)
                .sum();

        Ok(balance)

    }.await;

    tx.rollback()
        .await
        .map_err(|e| {
            AppError::Infrastructure(
                e.to_string()
            )
        })?;

    result
}
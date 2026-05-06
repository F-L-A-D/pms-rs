use crate::db::connection::Db;

use crate::repository::sqlite::folio_entry_repository::SqliteFolioEntryRepository;

pub async fn calculate_balance(
    db: &Db,
    folio_id: &str,
) -> Result<i64, String> {

    let entries =
        SqliteFolioEntryRepository::find_by_folio_id(
            &db.pool,
            folio_id,
        )
        .await?;

    let balance =
        entries
            .iter()
            .map(|e| e.amount)
            .sum();

    Ok(balance)
}
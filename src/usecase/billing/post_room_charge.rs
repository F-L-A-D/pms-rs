use crate::db::connection::Db;

use crate::domain::folio_entry::{
    FolioEntry,
    EntryType,
};

use crate::repository::sqlite::folio_repository::SqliteFolioRepository;
use crate::repository::sqlite::folio_entry_repository::SqliteFolioEntryRepository;

pub async fn post_room_charge(
    db: &Db,
    entry_id: String,
    folio_id: &str,
    amount: i64,
    description: Option<String>,
) -> Result<(), String> {

    let folio =
        SqliteFolioRepository::find_by_id(
            &db.pool,
            folio_id,
        )
        .await?;

    if folio.is_none() {
        return Err("folio not found".into());
    }

    let entry = FolioEntry::new(
        entry_id,
        folio_id.into(),
        EntryType::RoomCharge,
        amount,
        description,
    );

    SqliteFolioEntryRepository::save(
        &db.pool,
        &entry,
    )
    .await?;

    Ok(())
}
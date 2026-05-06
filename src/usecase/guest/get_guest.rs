use crate::db::connection::Db;

use crate::domain::guest::Guest;

use crate::repository::sqlite::guest_repository::SqliteGuestRepository;

pub async fn get_guest(
    db: &Db,
    guest_id: &str,
) -> Result<Option<Guest>, String> {

    SqliteGuestRepository::find_by_id(
        &db.pool,
        guest_id,
    )
    .await
}
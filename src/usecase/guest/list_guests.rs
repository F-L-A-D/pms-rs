use crate::db::connection::Db;

use crate::domain::guest::Guest;

use crate::repository::sqlite::guest_repository::SqliteGuestRepository;

pub async fn list_guests(
    db: &Db,
) -> Result<Vec<Guest>, String> {

    SqliteGuestRepository::find_all(
        &db.pool,
    )
    .await
}
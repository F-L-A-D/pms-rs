use crate::db::connection::Db;

use crate::domain::guest::Guest;

use crate::repository::sqlite::guest_repository::SqliteGuestRepository;

pub async fn create_guest(
    db: &Db,
    guest: Guest,
) -> Result<(), String> {

    SqliteGuestRepository::save(
        &db.pool,
        &guest,
    )
    .await
}
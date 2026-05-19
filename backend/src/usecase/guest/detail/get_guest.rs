use uuid::Uuid;

use crate::{
    db::connection::Db, domain::entity::guest::Guest, error::app_error::AppResult,
    repository::sqlite::operational::guest_repository::SqliteGuestRepository,
};

pub async fn get_guest(db: &Db, guest_id: Uuid) -> AppResult<Option<Guest>> {
    let mut tx = db.begin_tx().await;

    let guest = SqliteGuestRepository::find_by_id(&mut tx, guest_id).await?;

    let _ = tx.rollback().await;

    Ok(guest)
}

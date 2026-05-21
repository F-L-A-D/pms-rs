use uuid::Uuid;

use crate::{
    db::connection::Db, domain::entity::reservation::Reservation, error::app_error::AppResult,
    repository::sqlite::operational::reservation::reservation_repository::SqliteReservationRepository,
};

pub async fn get_guest_reservations(db: &Db, guest_id: Uuid) -> AppResult<Vec<Reservation>> {
    let mut tx = db.begin_tx().await;

    let result = SqliteReservationRepository::find_by_guest_id(&mut tx, guest_id).await;

    let _ = tx.rollback().await;

    result
}

use uuid::Uuid;

use crate::{
    db::connection::Db, domain::entity::reservation::Reservation, error::app_error::AppResult,
    repository::sqlite::operational::reservation_repository::SqliteReservationRepository,
};

pub async fn get_reservation(db: &Db, reservation_id: Uuid) -> AppResult<Option<Reservation>> {
    let mut tx = db.begin_tx().await;

    let result = SqliteReservationRepository::find_by_id(&mut tx, reservation_id).await;

    let _ = tx.rollback().await;

    result
}

use crate::db::connection::Db;

use crate::domain::reservation::{
    ReservationStatus,
    StayStatus,
};

use crate::repository::sqlite::repository::SqliteReservationRepository;
use crate::repository::sqlite::room_repository::SqliteRoomRepository;

pub async fn check_out(
    db: &Db,
    reservation_id: &str,
) -> Result<(), String> {

    let mut tx = db.begin_tx().await;

    let result = async {

        let mut reservation =
            SqliteReservationRepository::find_by_id_tx(
                &mut tx,
                reservation_id,
            )
            .await?
            .ok_or("reservation not found")?;

        if reservation.reservation_status != ReservationStatus::Active {
            return Err("reservation inactive".into());
        }

        if reservation.stay_status != Some(StayStatus::CheckedIn) {
            return Err("invalid stay status".into());
        }

        let room_id =
            reservation
                .room_id
                .clone()
                .ok_or("room not assigned")?;

        let mut room =
            SqliteRoomRepository::find_by_id_tx(
                &mut tx,
                &room_id,
            )
            .await?
            .ok_or("room not found")?;

        room.check_out()?;

        reservation.stay_status =
            Some(StayStatus::CheckedOut);

        SqliteRoomRepository::save_tx(
            &mut tx,
            &room,
        )
        .await?;

        SqliteReservationRepository::save_tx(
            &mut tx,
            &reservation,
        )
        .await?;

        Ok(())

    }.await;

    match result {

        Ok(_) => {
            tx.commit().await.unwrap();
            Ok(())
        }

        Err(e) => {
            tx.rollback().await.unwrap();
            Err(e)
        }
    }
}
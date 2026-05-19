use uuid::Uuid;

use crate::{
    db::connection::Db,
    domain::semantic::room_daily_state::RoomDailyOccupancyStatus,
    error::app_error::{conflict, infra, not_found, AppResult},
    repository::sqlite::operational::{
        reservation_repository::SqliteReservationRepository,
        room_daily_state_repository::SqliteRoomDailyStateRepository,
        room_repository::SqliteRoomRepository,
    },
};

pub async fn execute(db: &Db, reservation_id: Uuid, room_id: Uuid) -> AppResult<()> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let mut reservation = SqliteReservationRepository::find_by_id(&mut tx, reservation_id)
            .await?
            .ok_or_else(|| not_found("reservation not found"))?;

        let room = SqliteRoomRepository::find_by_id(&mut tx, room_id)
            .await?
            .ok_or_else(|| not_found("room not found"))?;

        if reservation.room_id.is_some() {
            return Err(conflict("already assigned"));
        }

        if !room.is_active {
            return Err(conflict("room inactive"));
        }

        if !room.is_physical {
            return Err(conflict("room is not physical"));
        }

        if room.room_class != reservation.room_class {
            return Err(conflict("room class mismatch"));
        }

        for service_date in reservation.nights() {
            if let Some(state) = SqliteRoomDailyStateRepository::find_by_room_and_service_date(
                &mut tx,
                room_id,
                service_date,
            )
            .await?
            {
                match state.occupancy_status {
                    RoomDailyOccupancyStatus::OutOfOrder => {
                        return Err(conflict("room out of order"));
                    }

                    RoomDailyOccupancyStatus::Occupied => {
                        return Err(conflict("room occupied"));
                    }

                    RoomDailyOccupancyStatus::Vacant => {}
                }
            }
        }

        reservation.room_id = Some(room_id);

        SqliteReservationRepository::modify(&mut tx, &mut reservation).await?;

        Ok(())
    }
    .await;

    match result {
        Ok(()) => {
            tx.commit().await.map_err(infra)?;

            Ok(())
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}

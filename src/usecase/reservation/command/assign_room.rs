use uuid::Uuid;

use crate::{
    db::connection::Db,

    domain::room::OccupancyStatus,

    error::app_error::{
        AppResult,
        conflict,
        infra,
        not_found,
    },

    repository::sqlite::operational::{
        reservation_repository::SqliteReservationRepository,
        room_repository::SqliteRoomRepository,
    },
};

pub async fn assign_room(
    db: &Db,
    reservation_id: Uuid,
    room_id: Uuid,
) -> AppResult<()> {

    let mut tx =
        db.begin_tx().await;

    let result = async {

        let mut reservation =
            SqliteReservationRepository
                ::find_by_id(
                    &mut tx,
                    reservation_id,
                )
                .await?
                .ok_or(
                    not_found(
                        "reservation not found"
                    )
                )?;

        let room =
            SqliteRoomRepository
                ::find_by_id(
                    &mut tx,
                    room_id,
                )
                .await?
                .ok_or(
                    not_found(
                        "room not found"
                    )
                )?;

        if reservation.room_id.is_some() {

            return Err(
                conflict(
                    "already assigned"
                )
            );
        }

        if room.occupancy_status
            != OccupancyStatus::Vacant
        {

            return Err(
                conflict(
                    "room not vacant"
                )
            );
        }

        reservation.room_id = Some(room_id);

        SqliteReservationRepository
            ::modify(
                &mut tx,
                &reservation,
            )
            .await?;

        Ok(())

    }.await;

    match result {

        Ok(_) => {

            tx.commit()
                .await
                .map_err(infra)?;

            Ok(())
        }

        Err(e) => {

            let _ =
                tx.rollback().await;

            Err(e)
        }
    }
}
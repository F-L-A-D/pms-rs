use crate::db::connection::Db;

use crate::domain::room::OccupancyStatus;

use crate::error::app_error::{
    AppError,
    AppResult,
};

use crate::repository::sqlite::operational::{
    reservation_repository::SqliteReservationRepository,
    room_repository::SqliteRoomRepository,
};

pub async fn assign_room(
    db: &Db,
    reservation_id: &str,
    room_id: &str,
) -> AppResult<()> {

    let mut tx =
        db.begin_tx().await;

    let result = async {

        let mut res =
            SqliteReservationRepository::find_by_id(
                &mut tx,
                reservation_id,
            )
            .await
            .map_err(AppError::Infrastructure)?
            .ok_or(
                AppError::NotFound(
                    "reservation not found".into()
                )
            )?;

        let room =
            SqliteRoomRepository::find_by_id(
                &mut tx,
                room_id,
            )
            .await
            .map_err(AppError::Infrastructure)?
            .ok_or(
                AppError::NotFound(
                    "room not found".into()
                )
            )?;

        if res.room_id.is_some() {

            return Err(
                AppError::Conflict(
                    "already assigned".into()
                )
            );
        }

        if room.occupancy_status !=
            OccupancyStatus::Vacant {

            return Err(
                AppError::Conflict(
                    "room not vacant".into()
                )
            );
        }

        res.room_id =
            Some(room_id.to_string());

        SqliteReservationRepository::update(
            &mut tx,
            &res,
        )
        .await
        .map_err(AppError::Infrastructure)?;

        Ok(())

    }.await;

    match result {

        Ok(_) => {

            tx.commit()
                .await
                .map_err(|e| {
                    AppError::Infrastructure(
                        e.to_string()
                    )
                })?;

            Ok(())
        }

        Err(e) => {

            tx.rollback()
                .await
                .map_err(|e| {
                    AppError::Infrastructure(
                        e.to_string()
                    )
                })?;

            Err(e)
        }
    }
}
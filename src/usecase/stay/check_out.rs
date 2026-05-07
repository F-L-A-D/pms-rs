use crate::db::connection::Db;

use crate::domain::reservation::{
    ReservationStatus,
    StayStatus,
};

use crate::error::app_error::{
    AppError,
    AppResult,
};

use crate::repository::sqlite::{
    reservation_repository::SqliteReservationRepository,
    room_repository::SqliteRoomRepository,
};

use crate::domain::guest_timeline_event::TimelineEventType;

use crate::usecase::timeline::record_event::record_event;

pub async fn check_out(
    db: &Db,
    reservation_id: &str,
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

        if res.reservation_status !=
            ReservationStatus::Active {

            return Err(
                AppError::Conflict(
                    "reservation inactive".into()
                )
            );
        }

        if res.stay_status !=
            Some(StayStatus::CheckedIn) {

            return Err(
                AppError::Conflict(
                    "invalid stay status".into()
                )
            );
        }

        let room_id =
            res
                .room_id
                .clone()
                .ok_or(
                    AppError::Conflict(
                        "room not assigned".into()
                    )
                )?;

        let mut room =
            SqliteRoomRepository::find_by_id(
                &mut tx,
                &room_id,
            )
            .await
            .map_err(AppError::Infrastructure)?
            .ok_or(
                AppError::NotFound(
                    "room not found".into()
                )
            )?;

        room.check_out()
            .map_err(AppError::Conflict)?;

        res.stay_status =
            Some(StayStatus::CheckedOut);

        SqliteRoomRepository::save(
            &mut tx,
            &room,
        )
        .await
        .map_err(AppError::Infrastructure)?;

        SqliteReservationRepository::save(
            &mut tx,
            &res,
        )
        .await
        .map_err(AppError::Infrastructure)?;
        
        if let Some(guest_id) =
            &res.primary_guest_id {

            record_event(
                &mut tx,
                guest_id.clone(),
                TimelineEventType::CheckedOut,
                res.id.clone(),
            )
            .await?;
        }

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
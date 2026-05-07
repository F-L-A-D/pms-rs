use crate::db::connection::Db;

use crate::domain::reservation::ReservationStatus;

use crate::domain::guest_timeline_event::TimelineEventType;

use crate::error::app_error::{
    AppError,
    AppResult,
};

use crate::repository::sqlite::{
    inventory_repository::SqliteInventoryRepository,
    reservation_repository::SqliteReservationRepository
};

use crate::usecase::timeline::record_event::record_event;

pub async fn cancel_reservation(
    db: &Db,
    id: &str,
) -> AppResult<()> {

    let mut tx =
        db.begin_tx().await;

    let result = async {

        let mut res =
            SqliteReservationRepository::find_by_id(
                &mut tx,
                id,
            )
            .await
            .map_err(AppError::Infrastructure)?
            .ok_or(
                AppError::NotFound(
                    "reservation not found".into()
                )
            )?;

        if res.reservation_status ==
            ReservationStatus::Cancelled {

            return Ok(());
        }

        let dates =
            res.nights();

        for d in dates {

            SqliteInventoryRepository::add(
                &mut tx,
                &d.to_string(),
                -1,
                10,
            )
            .await
            .map_err(AppError::Infrastructure)?;
        }

        res.reservation_status =
            ReservationStatus::Cancelled;

        res.stay_status = None;

        let primary_guest_id =
            res
                .primary_participant()
                .map(|p| p.guest_id.clone());

        SqliteReservationRepository::update(
            &mut tx,
            &res,
        )
        .await
        .map_err(AppError::Infrastructure)?;

        if let Some(guest_id) =
            &primary_guest_id {

            record_event(
                &mut tx,
                guest_id.clone(),
                TimelineEventType::ReservationCancelled,
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
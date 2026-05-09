use crate::db::connection::Db;

use uuid::Uuid;

use crate::domain::reservation::ReservationStatus;

use crate::domain::guest_timeline_event::TimelineEventType;

use crate::error::app_error::{AppError, AppResult};

use crate::repository::sqlite::operational::
    reservation_repository::SqliteReservationRepository;

use crate::usecase::timeline::record_event::record_event;

use crate::projection::service::
    inventory_projection_service::remove_reservation_projection;

pub async fn cancel_reservation(db: &Db, id: Uuid) -> AppResult<()> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let mut reservation = SqliteReservationRepository::find_by_id(&mut tx, id)
            .await
            .map_err(AppError::Infrastructure)?
            .ok_or(AppError::NotFound("reservation not found".into()))?;

        if reservation.reservation_status == ReservationStatus::Cancelled {
            return Ok(());
        }

        remove_reservation_projection(
            &mut tx,
            &reservation,
        )
        .await?;

        reservation.reservation_status = ReservationStatus::Cancelled;

        reservation.stay_status = None;

        let primary_guest_id = 
            reservation.primary_participant().map(|p| p.guest_id.clone());

        SqliteReservationRepository::modify(&mut tx, &reservation)
            .await
            .map_err(AppError::Infrastructure)?;

        if let Some(guest_id) = &primary_guest_id {
            record_event(
                &mut tx,
                guest_id.clone(),
                TimelineEventType::ReservationCancelled,
                reservation.id,
            )
            .await?;
        }

        Ok(())
    }
    .await;

    match result {
        Ok(_) => {
            tx.commit()
                .await
                .map_err(|e| AppError::Infrastructure(e.to_string()))?;

            Ok(())
        }

        Err(e) => {
            tx.rollback()
                .await
                .map_err(|e| AppError::Infrastructure(e.to_string()))?;

            Err(e)
        }
    }
}

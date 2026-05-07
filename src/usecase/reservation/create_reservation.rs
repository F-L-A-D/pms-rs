use crate::db::connection::Db;

use crate::domain::reservation::Reservation;

use crate::error::app_error::{
    AppError,
    AppResult,
};

use crate::repository::sqlite::{
    guest_repository::SqliteGuestRepository,
    reservation_guest_relation_repository::
        SqliteReservationGuestRelationRepository,
    reservation_repository::SqliteReservationRepository,
};

use crate::domain::guest_timeline_event::TimelineEventType;

use crate::usecase::timeline::record_event::record_event;

pub async fn create_reservation(
    db: &Db,
    reservation: Reservation,
) -> AppResult<()> {

    let mut tx =
        db.begin_tx().await;

    let result = async {

        for participant in &reservation.participants {

            let guest =
                SqliteGuestRepository::find_by_id(
                    &mut tx,
                    &participant.guest_id,
                )
                .await
                .map_err(AppError::Infrastructure)?;

            if guest.is_none() {

                return Err(
                    AppError::NotFound(
                        format!(
                            "guest not found: {}",
                            participant.guest_id
                        )
                    )
                );
            }
        }

        SqliteReservationRepository::save(
            &mut tx,
            &reservation,
        )
        .await
        .map_err(AppError::Infrastructure)?;

        for participant in &reservation.participants {

            SqliteReservationGuestRelationRepository::save(
                &mut tx,
                participant,
            )
            .await
            .map_err(AppError::Infrastructure)?;
        }

        let primary_guest_id =
            reservation
                .primary_participant()
                .map(|p| p.guest_id.clone());

        if let Some(guest_id) =
            &primary_guest_id {

            record_event(
                &mut tx,
                guest_id.clone(),
                TimelineEventType::ReservationCreated,
                reservation.id.clone(),
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
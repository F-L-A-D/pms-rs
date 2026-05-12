use uuid::Uuid;

use crate::{
    db::connection::Db,

    domain::{
        guest_timeline_event::TimelineEventType,
        reservation::{Reservation, ReservationStatus},
    },

    error::app_error::{
        AppResult,
        infra,
        not_found,
    },

    projection::service::inventory_projection_service::
        remove_reservation_projection,

    repository::sqlite::operational::
        reservation_repository::SqliteReservationRepository,

    usecase::timeline::command::record_event::record_event,
};

pub async fn cancel_reservation(
    db: &Db,
    id: Uuid,
) -> AppResult<Reservation> {

    let mut tx =
        db.begin_tx().await;

    let result = async {

        let mut reservation =
            SqliteReservationRepository
                ::find_by_id(
                    &mut tx,
                    id,
                )
                .await?
                .ok_or(
                    not_found(
                        "reservation not found"
                    )
                )?;

        if reservation.reservation_status
            == ReservationStatus::Cancelled{
                return Ok(reservation);
        }

        remove_reservation_projection(
            &mut tx,
            &reservation,
        )
        .await?;

        reservation.reservation_status =
            ReservationStatus::Cancelled;

        reservation.stay_status =
            None;

        let primary_guest_id =
            reservation
                .primary_participant()
                .map(|p| p.guest_id);

        SqliteReservationRepository
            ::modify(
                &mut tx,
                &reservation,
            )
            .await?;

        if let Some(guest_id) = primary_guest_id {

            record_event(
                &mut tx,
                guest_id,
                TimelineEventType::ReservationCancelled,
                reservation.id,
            )
            .await?;
        }

        Ok(reservation)

    }.await;

    match result {

        Ok(reservation) => {

            tx.commit()
                .await
                .map_err(infra)?;

            Ok(reservation)
        }

        Err(e) => {

            let _ =
                tx.rollback().await;

            Err(e)
        }
    }
}
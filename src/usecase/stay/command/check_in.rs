use uuid::Uuid;

use crate::{
    db::connection::Db,

    domain::{
        guest_timeline_event::TimelineEventType,

        reservation::{
            ReservationStatus,
            StayStatus,
        },
    },

    error::app_error::{
        AppResult,
        conflict,
        infra,
        not_found,
    },

    repository::sqlite::operational::{
        reservation_repository::
            SqliteReservationRepository,

        room_repository::
            SqliteRoomRepository,
    },

    usecase::timeline::command::record_event::record_event,
};

pub async fn check_in(
    db: &Db,
    reservation_id: Uuid,
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

        if reservation.reservation_status
            != ReservationStatus::Active
        {

            return Err(
                conflict(
                    "reservation inactive"
                )
            );
        }

        if reservation.stay_status
            != Some(StayStatus::Confirmed)
        {

            return Err(
                conflict(
                    "invalid stay status"
                )
            );
        }

        let room_id =
            reservation
                .room_id
                .ok_or(
                    conflict(
                        "room not assigned"
                    )
                )?;

        let mut room =
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

        room.check_in()
            .map_err(conflict)?;

        reservation.stay_status =
            Some(
                StayStatus::CheckedIn
            );

        SqliteRoomRepository
            ::save(
                &mut tx,
                &room,
            )
            .await?;

        SqliteReservationRepository
            ::modify(
                &mut tx,
                &reservation,
            )
            .await?;

        let primary_guest_id =
            reservation
                .primary_participant()
                .map(|p| p.guest_id);

        if let Some(guest_id)
            = primary_guest_id
        {

            record_event(
                &mut tx,
                guest_id,
                TimelineEventType::CheckedIn,
                reservation.id,
            )
            .await?;
        }

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
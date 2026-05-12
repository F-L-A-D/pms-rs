use crate::{
    db::connection::Db,

    error::app_error::{
        AppResult,
        infra,
        not_found,
    },
    
    domain::{
        guest_timeline_event::TimelineEventType,
        reservation::Reservation,
    },

    usecase::timeline::command::record_event::record_event,

    repository::sqlite::operational::{
        guest_repository::
            SqliteGuestRepository,
        reservation_guest_relation_repository::
            SqliteReservationGuestRelationRepository,
        reservation_repository::
            SqliteReservationRepository,
    },

    projection::service::{
        guest_summary_projection_service::refresh_guest_summary_projection,
        inventory_projection_service::apply_reservation_projection,
    },
};

pub async fn create_reservation(
    db: &Db,
    reservation: Reservation,
) -> AppResult<()> {

    let mut tx =
        db.begin_tx().await;

    let result = async {

        for participant in &reservation.participants {

            let guest =
                SqliteGuestRepository
                    ::find_by_id(
                        &mut tx,
                        participant.guest_id,
                    )
                    .await?;

            if guest.is_none() {
                return Err(
                    not_found(
                        format!(
                            "guest not found: {}",
                            participant.guest_id,
                        )
                    )
                );
            }
        }

        SqliteReservationRepository
            ::save(
                &mut tx,
                &reservation,
            )
            .await?;

        for participant in &reservation.participants {

            SqliteReservationGuestRelationRepository
                ::save(
                    &mut tx,
                    participant,
                )
                .await?;
        }

        let primary_guest_id =
            reservation
                .primary_participant()
                .map(|p| p.guest_id);

        if let Some(guest_id) = primary_guest_id {

            record_event(
                &mut tx,
                guest_id,
                TimelineEventType::ReservationCreated,
                reservation.id,
            )
            .await?;
        }

        apply_reservation_projection(
            &mut tx,
            &reservation,
        )
        .await?;

        for participant in &reservation.participants {

            refresh_guest_summary_projection(
                &mut tx,
                participant.guest_id,
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
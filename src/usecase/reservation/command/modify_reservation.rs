use uuid::Uuid;

use crate::{
    api::dto::input::reservation::ModifyReservationInput,
    db::connection::Db,
    domain::{
        entity::reservation::Reservation,
        semantic::reservation_semantics::detect_reservation_timeline_events,
    },
    error::app_error::{infra, not_found, validation, AppResult},
    projection::{
        invalidation::{
            projection_invalidation::{ProjectionInvalidation, ProjectionRefreshTarget},
            projection_scope::ProjectionScope,
        },
        orchestrator::refresh_projection_chain::refresh_projection_chain,
        topology::projection_node::ProjectionNode,
    },
    repository::sqlite::operational::reservation_repository::SqliteReservationRepository,
    usecase::timeline::command::record_event::record_event,
};

pub async fn modify_reservation(
    db: &Db,
    id: Uuid,
    input: ModifyReservationInput,
) -> AppResult<Reservation> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let mut reservation = SqliteReservationRepository::find_by_id(&mut tx, id)
            .await?
            .ok_or(not_found("reservation not found"))?;

        let before = reservation.clone();

        reservation.check_in = input.check_in.unwrap_or(reservation.check_in);

        reservation.check_out = input.check_out.unwrap_or(reservation.check_out);

        reservation.room_class = input.room_class.unwrap_or(reservation.room_class.clone());

        if reservation.check_in > reservation.check_out {
            return Err(validation("check_in must be <= check_out"));
        }

        let timeline_event_types = detect_reservation_timeline_events(&before, &reservation);

        SqliteReservationRepository::modify(&mut tx, &reservation).await?;

        for participant in &reservation.participants {
            for event_type in &timeline_event_types {
                record_event(
                    &mut tx,
                    participant.guest_id,
                    event_type.clone(),
                    reservation.id,
                )
                .await?;
            }
        }

        for participant in &reservation.participants {
            refresh_projection_chain(
                &mut tx,
                ProjectionInvalidation::new(
                    ProjectionNode::GuestAggregate,
                    ProjectionScope::Guest,
                    ProjectionRefreshTarget::Guest {
                        guest_id: participant.guest_id,
                    },
                ),
            )
            .await?;
        }

        Ok(reservation)
    }
    .await;

    match result {
        Ok(reservation) => {
            tx.commit().await.map_err(infra)?;

            Ok(reservation)
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}

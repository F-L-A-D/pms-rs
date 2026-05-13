use crate::{
    db::connection::Db,

    domain::{
        guest_timeline_event::TimelineEventType,
        reservation::Reservation,
    },

    error::app_error::{
        AppResult,
        infra,
        not_found,
    },

    projection::{
        invalidation::{
            projection_invalidation::{
                ProjectionInvalidation,
                ProjectionRefreshTarget,
            },

            projection_scope::
                ProjectionScope,
        },

        orchestrator::refresh_projection_chain::
            refresh_projection_chain,

        topology::projection_node::
            ProjectionNode,
    },

    repository::sqlite::operational::{
        guest_repository::
            SqliteGuestRepository,

        reservation_guest_relation_repository::
            SqliteReservationGuestRelationRepository,

        reservation_repository::
            SqliteReservationRepository,
    },

    usecase::timeline::command::
        record_event::record_event,
};

pub async fn create_reservation(
    db: &Db,
    reservation: Reservation,
) -> AppResult<()>
{
    let mut tx =
        db.begin_tx().await;

    let result = async {

        for participant in
            &reservation.participants
        {
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
                        ),
                    ),
                );
            }
        }

        SqliteReservationRepository
            ::save(
                &mut tx,
                &reservation,
            )
            .await?;

        for participant in
            &reservation.participants
        {
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

        if let Some(guest_id) =
            primary_guest_id
        {
            record_event(
                &mut tx,
                guest_id,
                TimelineEventType::
                    ReservationCreated,
                reservation.id,
            )
            .await?;
        }

        for participant in
            &reservation.participants
        {
            refresh_projection_chain(
                &mut tx,

                ProjectionInvalidation::new(
                    ProjectionNode::GuestAggregate,

                    ProjectionScope::Guest,

                    ProjectionRefreshTarget::Guest {
                        guest_id:
                            participant.guest_id,
                    },
                ),
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
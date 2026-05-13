use chrono::NaiveDate;

use uuid::Uuid;

use crate::{
    db::connection::Db,

    domain::reservation::
        Reservation,

    error::app_error::{
        AppResult,
        infra,
        not_found,
        validation,
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

    repository::sqlite::operational::
        reservation_repository::
            SqliteReservationRepository,
};

pub async fn modify_reservation(
    db: &Db,
    id: Uuid,
    check_in: Option<NaiveDate>,
    check_out: Option<NaiveDate>,
    room_class: Option<String>,
) -> AppResult<Reservation>
{
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
                        "reservation not found",
                    ),
                )?;

        let check_in =
            check_in.unwrap_or(
                reservation.check_in,
            );

        let check_out =
            check_out.unwrap_or(
                reservation.check_out,
            );

        if check_in > check_out {

            return Err(
                validation(
                    "check_in must be <= check_out",
                ),
            );
        }

        let room_class =
            room_class.unwrap_or(
                reservation
                    .room_class
                    .clone(),
            );

        reservation.check_in =
            check_in;

        reservation.check_out =
            check_out;

        reservation.room_class =
            room_class;

        SqliteReservationRepository
            ::modify(
                &mut tx,
                &reservation,
            )
            .await?;

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
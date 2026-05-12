use uuid::Uuid;

use crate::{
    db::connection::Db,

    error::app_error::{
        AppResult,
        infra,
        not_found,
        validation,
    },

    projection::service::
        inventory_projection_service::
            transition_reservation_projection,

    repository::sqlite::operational::
        reservation_repository::SqliteReservationRepository,

    usecase::reservation::stay_input::{
        normalize,
        StayInput,
    },
};

pub async fn modify_reservation(
    db: &Db,
    id: Uuid,
    input: StayInput,
) -> AppResult<()> {

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

        let old_reservation =
            reservation.clone();

        let (check_in, check_out) =
            normalize(
                input.clone()
            )
            .map_err(validation)?;

        reservation.check_in =
            check_in;

        reservation.check_out =
            check_out;

        reservation.room_class =
            input.room_class();

        SqliteReservationRepository
            ::modify(
                &mut tx,
                &reservation,
            )
            .await?;

        transition_reservation_projection(
            &mut tx,
            &old_reservation,
            &reservation,
        )
        .await?;

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
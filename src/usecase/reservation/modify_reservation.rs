use crate::db::connection::Db;

use uuid::Uuid;

use crate::usecase::reservation::stay_input::{
    normalize,
    StayInput,
};

use crate::error::app_error::{
    AppError,
    AppResult,
};

use crate::repository::sqlite::operational::
    reservation_repository::SqliteReservationRepository;


use crate::projection::service::
    inventory_projection_service::transition_reservation_projection;

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
                .await
                .map_err(
                    AppError::Infrastructure
                )?
                .ok_or(
                    AppError::NotFound(
                        "reservation not found"
                            .into()
                    )
                )?;

        let old_reservation =
            reservation.clone();

        let (check_in, check_out) =
            normalize(
                input.clone()
            )
            .map_err(
                AppError::Validation
            )?;

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
            .await
            .map_err(
                AppError::Infrastructure
            )?;

        transition_reservation_projection(
            &mut tx,
            &old_reservation,
            &reservation,
        )
        .await?;

        Ok(())
    }
    .await;

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
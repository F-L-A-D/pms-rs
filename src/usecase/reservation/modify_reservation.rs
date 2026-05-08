use crate::db::connection::Db;

use crate::usecase::reservation::stay_input::{
    normalize,
    StayInput,
};

use crate::error::app_error::{
    AppError,
    AppResult,
};

use crate::repository::sqlite::operational::{
    inventory_repository::SqliteInventoryRepository,
    reservation_repository::SqliteReservationRepository,
};

pub async fn modify_reservation(
    db: &Db,
    id: &str,
    input: StayInput,
) -> AppResult<()> {

    let mut tx =
        db.begin_tx().await;

    let result = async {

        let mut res =
            SqliteReservationRepository::find_by_id(
                &mut tx,
                id,
            )
            .await
            .map_err(AppError::Infrastructure)?
            .ok_or(
                AppError::NotFound(
                    "reservation not found".into()
                )
            )?;

        let old_dates =
            res.nights();

        let (check_in, check_out) =
            normalize(input)
                .map_err(AppError::Validation)?;

        res.check_in = check_in;
        res.check_out = check_out;

        SqliteReservationRepository::update(
            &mut tx,
            &res,
        )
        .await
        .map_err(AppError::Infrastructure)?;

        let new_dates =
            res.nights();

        for d in old_dates {

            SqliteInventoryRepository::add(
                &mut tx,
                &d.to_string(),
                -1,
                10,
            )
            .await
            .map_err(AppError::Infrastructure)?;
        }

        for d in new_dates {

            SqliteInventoryRepository::add(
                &mut tx,
                &d.to_string(),
                1,
                10,
            )
            .await
            .map_err(AppError::Infrastructure)?;
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
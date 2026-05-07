use crate::db::connection::Db;

use crate::adapter::stay_input::{
    normalize,
    StayInput,
};

use crate::domain::reservation::Reservation;

use crate::domain::reservation_guest_relation::{
    ReservationGuestRelation,
    ReservationGuestRelationType,
};

use crate::error::app_error::{
    AppError,
    AppResult,
};

use crate::repository::sqlite::guest_repository::SqliteGuestRepository;
use crate::repository::sqlite::inventory_repository::SqliteInventoryRepository;
use crate::repository::sqlite::reservation_guest_relation_repository::SqliteReservationGuestRelationRepository;
use crate::repository::sqlite::reservation_repository::SqliteReservationRepository;

pub async fn create_reservation(
    db: &Db,
    id: String,
    input: StayInput,
    primary_guest_id: Option<String>,
) -> AppResult<()> {

    let mut tx =
        db.begin_tx().await;

    let result = async {

        let (check_in, check_out) =
            normalize(input.clone())
                .map_err(AppError::Validation)?;

        let room_class =
            input.room_class();

        if let Some(guest_id) =
            &primary_guest_id {

            let guest =
                SqliteGuestRepository::find_by_id(
                    &mut tx,
                    guest_id,
                )
                .await
                .map_err(AppError::Infrastructure)?;

            if guest.is_none() {

                return Err(
                    AppError::NotFound(
                        "guest not found".into()
                    )
                );
            }
        }

        let reservation =
            Reservation::new(
                id,
                check_in,
                check_out,
                room_class,
                primary_guest_id.clone(),
            )
            .map_err(AppError::Validation)?;

        SqliteReservationRepository::save(
            &mut tx,
            &reservation,
        )
        .await
        .map_err(AppError::Infrastructure)?;

        if let Some(guest_id) =
            &primary_guest_id {

            let relation =
                ReservationGuestRelation {
                    reservation_id:
                        reservation.id.clone(),

                    guest_id:
                        guest_id.clone(),

                    relation_type:
                        ReservationGuestRelationType::Primary,
                };

            SqliteReservationGuestRelationRepository::save(
                &mut tx,
                &relation,
            )
            .await
            .map_err(AppError::Infrastructure)?;
        }

        for d in reservation.nights() {

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
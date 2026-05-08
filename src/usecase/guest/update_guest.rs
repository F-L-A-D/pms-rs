use chrono::NaiveDate;

use uuid::Uuid;

use crate::db::connection::Db;

use crate::domain::guest::{
    Guest,
    Gender,
};

use crate::error::app_error::{
    AppError,
    AppResult,
};

use crate::repository::sqlite::operational::
    guest_repository::SqliteGuestRepository;

pub async fn update_guest(
    db: &Db,
    guest_id: Uuid,
    last_name: String,
    first_name: String,
    phone: Option<String>,
    email: Option<String>,
    nationality: Option<String>,
    birth_date: Option<NaiveDate>,
    gender: Option<Gender>,
    membership_code: Option<String>,
    marketing_opt_in: bool,
) -> AppResult<Guest> {

    let mut tx =
        db.begin_tx().await;

    let result = async {

        let mut guest =
            SqliteGuestRepository::find_by_id(
                &mut tx,
                guest_id,
            )
            .await
            .map_err(AppError::Infrastructure)?
            .ok_or(
                AppError::NotFound(
                    "guest not found".into()
                )
            )?;

        guest.update_profile(
            last_name,
            first_name,
            phone,
            email,
            nationality,
            birth_date,
            gender,
            membership_code,
            marketing_opt_in,
        )
        .map_err(AppError::Validation)?;

        SqliteGuestRepository::update(
            &mut tx,
            &guest,
        )
        .await
        .map_err(AppError::Infrastructure)?;

        Ok(guest)

    }.await;

    match result {

        Ok(guest) => {

            tx.commit()
                .await
                .map_err(|e| {
                    AppError::Infrastructure(
                        e.to_string()
                    )
                })?;

            Ok(guest)
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
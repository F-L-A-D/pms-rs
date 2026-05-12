use uuid::Uuid;

use crate::{
    db::connection::Db,

    domain::guest::{
        Guest,
        GuestProfileUpdate,
    },

    error::app_error::{
        AppResult,
        infra,
        not_found,
        validation,
    },

    repository::sqlite::operational::
        guest_repository::
            SqliteGuestRepository,
};

pub async fn update_guest(
    db: &Db,

    guest_id: Uuid,

    update: GuestProfileUpdate,
) -> AppResult<Guest> {

    let mut tx =
        db.begin_tx().await;

    let result = async {

        let mut guest =
            SqliteGuestRepository
                ::find_by_id(
                    &mut tx,
                    guest_id,
                )
                .await?
                .ok_or(
                    not_found(
                        "guest not found"
                    )
                )?;

        guest.update_profile(
            update,
        )
        .map_err(
            validation
        )?;

        SqliteGuestRepository
            ::update(
                &mut tx,
                &guest,
            )
            .await?;

        Ok(guest)

    }.await;

    match result {

        Ok(guest) => {

            tx.commit()
                .await
                .map_err(infra)?;

            Ok(guest)
        }

        Err(e) => {

            let _ =
                tx.rollback().await;

            Err(e)
        }
    }
}
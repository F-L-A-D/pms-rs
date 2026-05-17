use uuid::Uuid;

use crate::{
    api::dto::input::guest::CreateGuestInput,
    db::connection::Db,
    domain::entity::guest::{Guest, GuestProfile},
    error::app_error::{infra, validation, AppResult},
    repository::sqlite::operational::guest_repository::SqliteGuestRepository,
};

pub async fn create_guest(db: &Db, input: CreateGuestInput) -> AppResult<Guest> {
    let guest_id = Uuid::new_v4();

    let profile = GuestProfile::new(
        input.last_name,
        input.first_name,
        input.phone,
        input.email,
        input.nationality,
        input.birth_date,
        input.gender,
        input.membership_code,
        input.marketing_opt_in,
    )
    .map_err(validation)?;

    let guest = Guest::new(guest_id, profile);

    let mut tx = db.begin_tx().await;

    let result = async {
        SqliteGuestRepository::save(&mut tx, &guest).await?;

        Ok(())
    }
    .await;

    match result {
        Ok(_) => {
            tx.commit().await.map_err(infra)?;

            Ok(guest)
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}

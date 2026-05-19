use crate::{
    api::dto::input::guest::UpdateGuestInput,
    db::connection::Db,
    domain::entity::guest::{Guest, GuestProfile},
    error::app_error::{infra, not_found, validation, AppResult},
    repository::sqlite::operational::guest_repository::SqliteGuestRepository,
};

pub async fn update_guest(db: &Db, input: UpdateGuestInput) -> AppResult<Guest> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let mut guest = SqliteGuestRepository::find_by_id(&mut tx, input.guest_id)
            .await?
            .ok_or(not_found("guest not found"))?;

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

        guest.replace_profile(profile);

        SqliteGuestRepository::update(&mut tx, &guest).await?;

        Ok(guest)
    }
    .await;

    match result {
        Ok(guest) => {
            tx.commit().await.map_err(infra)?;

            Ok(guest)
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}

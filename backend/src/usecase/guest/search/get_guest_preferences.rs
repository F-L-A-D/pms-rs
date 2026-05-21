use uuid::Uuid;

use crate::{
    db::connection::Db,
    domain::semantic::guest_preference::GuestPreference,
    error::app_error::{not_found, AppResult},
    repository::sqlite::operational::guest::{
        guest_preference_repository::SqliteGuestPreferenceRepository,
        guest_repository::SqliteGuestRepository,
    },
};

pub async fn execute(db: &Db, guest_id: Uuid) -> AppResult<Vec<GuestPreference>> {
    let mut tx = db.begin_tx().await;

    let result = async {
        SqliteGuestRepository::find_by_id(&mut tx, guest_id)
            .await?
            .ok_or_else(|| not_found("guest not found"))?;

        SqliteGuestPreferenceRepository::list_by_guest_id(&mut tx, guest_id).await
    }
    .await;

    let _ = tx.rollback().await;

    result
}

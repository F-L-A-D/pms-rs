use crate::db::connection::Db;

use uuid::Uuid;

use crate::domain::guest::Guest;

use crate::error::app_error::{
    AppError,
    AppResult,
};

use crate::repository::sqlite::operational::
    guest_repository::SqliteGuestRepository;

pub async fn get_guest(
    db: &Db,
    guest_id: Uuid,
) -> AppResult<Option<Guest>> {

    let mut tx =
        db.begin_tx().await;

    let result = async {

        let guest =
            SqliteGuestRepository::find_by_id(
                &mut tx,
                guest_id,
            )
            .await
            .map_err(AppError::Infrastructure)?;

        Ok(guest)

    }.await;

    tx.rollback()
        .await
        .map_err(|e| {
            AppError::Infrastructure(
                e.to_string()
            )
        })?;

    result
}
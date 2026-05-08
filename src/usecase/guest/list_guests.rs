use crate::db::connection::Db;

use crate::domain::guest::Guest;

use crate::error::app_error::{
    AppError,
    AppResult,
};

use crate::repository::sqlite::operational::
    guest_repository::SqliteGuestRepository;

pub async fn list_guests(
    db: &Db,
    keyword: Option<String>,
) -> AppResult<Vec<Guest>> {

    let mut tx =
        db.begin_tx().await;

    let result = async {

        let guests =
            match keyword {

                Some(keyword) => {

                    SqliteGuestRepository::find_by_name(
                        &mut tx,
                        &keyword,
                    )
                    .await?
                }

                None => {

                    SqliteGuestRepository::find_all(
                        &mut tx,
                    )
                    .await?
                }
            };

        Ok(guests)

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
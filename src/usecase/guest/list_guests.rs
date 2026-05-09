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
    field: Option<String>,
) -> AppResult<Vec<Guest>> {

    let mut tx =
        db.begin_tx().await;

    let result = async {

        let guests =
            match keyword {

                Some(keyword) => {

                    SqliteGuestRepository::search(
                        &mut tx,
                        &keyword,
                        field.as_deref(),
                    )
                    .await?
                }

                None => {

                    SqliteGuestRepository::list(
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
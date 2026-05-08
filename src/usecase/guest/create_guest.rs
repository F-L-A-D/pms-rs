use crate::db::connection::Db;

use crate::domain::guest::Guest;

use crate::error::app_error::{
    AppError,
    AppResult,
};

use crate::repository::sqlite::operational::
    guest_repository::SqliteGuestRepository;

pub async fn create_guest(
    db: &Db,
    guest: Guest,
) -> AppResult<()> {

    let mut tx =
        db.begin_tx().await;

    let result = async {

        SqliteGuestRepository::save(
            &mut tx,
            &guest,
        )
        .await?;

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
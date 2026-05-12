use crate::{
    db::connection::Db,

    domain::guest::Guest,

    error::app_error::
        AppResult,

    repository::sqlite::operational::
        guest_repository::
            SqliteGuestRepository,
};

pub async fn get_guests(
    db: &Db,
    keyword: Option<String>,
    field: Option<String>,
) -> AppResult<Vec<Guest>> {

    let mut tx =
        db.begin_tx().await;

    let guests =
        match keyword {

            Some(keyword) => {

                SqliteGuestRepository
                    ::search(
                        &mut tx,
                        &keyword,
                        field.as_deref(),
                    )
                    .await?
            }

            None => {

                SqliteGuestRepository
                    ::list(
                        &mut tx,
                    )
                    .await?
            }
        };

    let _ =
        tx.rollback().await;

    Ok(guests)
}
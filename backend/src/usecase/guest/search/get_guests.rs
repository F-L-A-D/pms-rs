use crate::{
    api::dto::input::guest::GuestSearchInput, db::connection::Db, domain::entity::guest::Guest,
    error::app_error::AppResult,
    repository::sqlite::operational::guest_repository::SqliteGuestRepository,
};

pub async fn get_guests(db: &Db, input: GuestSearchInput) -> AppResult<Vec<Guest>> {
    let mut tx = db.begin_tx().await;

    let guests = match input.query {
        Some(keyword) => SqliteGuestRepository::search(&mut tx, &keyword, input.field).await?,

        None => SqliteGuestRepository::list_guests(&mut tx).await?,
    };

    let _ = tx.rollback().await;

    Ok(guests)
}

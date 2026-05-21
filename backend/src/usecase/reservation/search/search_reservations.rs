use crate::{
    api::dto::input::reservation::SearchReservationsInput,

    db::connection::Db,

    domain::semantic::reservation_search_item::ReservationSearchItem,

    error::app_error::AppResult,

    repository::sqlite::operational::reservation::reservation_repository::
        SqliteReservationRepository,
};

pub async fn execute(
    db: &Db,
    input: SearchReservationsInput,
) -> AppResult<Vec<ReservationSearchItem>> {
    let mut tx =
        db.begin_tx().await;

    let result =
        SqliteReservationRepository::find_by_search_input(
            &mut tx,
            &input,
        )
        .await;

    match result {
        Ok(items) => {
            let _ = tx.commit().await;
            Ok(items)
        }

        Err(error) => {
            let _ = tx.rollback().await;
            Err(error)
        }
    }
}
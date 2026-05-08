use crate::db::connection::Db;

use crate::domain::room::Room;

use crate::error::app_error::{
    AppError,
    AppResult,
};

use crate::repository::sqlite::operational::
    room_repository::SqliteRoomRepository;

pub async fn list_rooms(
    db: &Db,
) -> AppResult<Vec<Room>> {

    let mut tx =
        db.begin_tx().await;

    let result = async {

        let rooms =
            SqliteRoomRepository::find_all(
                &mut tx,
            )
            .await
            .map_err(AppError::Infrastructure)?;

        Ok(rooms)

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
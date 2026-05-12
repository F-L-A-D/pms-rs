use crate::{
    db::connection::Db,

    domain::room::Room,

    error::app_error::AppResult,

    repository::sqlite::operational::
        room_repository::SqliteRoomRepository,
};

pub async fn get_rooms(
    db: &Db,
) -> AppResult<Vec<Room>> {

    let mut tx =
        db.begin_tx().await;

    let result =
        SqliteRoomRepository
            ::find_all(
                &mut tx,
            )
            .await;

    let _ =
        tx.rollback().await;

    result
}
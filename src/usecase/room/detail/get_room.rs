use uuid::Uuid;

use crate::{
    db::connection::Db,

    domain::room::Room,

    error::app_error::AppResult,
    
    repository::sqlite::operational::
        room_repository::SqliteRoomRepository,
};

pub async fn get_room(
    db: &Db,
    room_id: Uuid,
) -> AppResult<Option<Room>> {

    let mut tx =
        db.begin_tx().await;

    let result =
        SqliteRoomRepository
            ::find_by_id(
                &mut tx,
                room_id,
            )
            .await;

    let _ =
        tx.rollback().await;

    result
}
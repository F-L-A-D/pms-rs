use crate::{
    api::dto::input::room::ListRoomsInput, db::connection::Db, domain::entity::room::Room,
    error::app_error::AppResult,
    repository::sqlite::operational::room_repository::SqliteRoomRepository,
};

pub async fn list_rooms(db: &Db, input: ListRoomsInput) -> AppResult<Vec<Room>> {
    let mut tx = db.begin_tx().await;

    let rooms = if input.include_inactive {
        SqliteRoomRepository::find_all(&mut tx).await?
    } else {
        SqliteRoomRepository::find_active(&mut tx).await?
    };

    let _ = tx.rollback().await;

    Ok(rooms)
}

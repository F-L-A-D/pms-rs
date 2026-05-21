use crate::{
    api::dto::input::room::GetRoomInput,
    db::connection::Db,
    domain::entity::room::Room,
    error::app_error::{not_found, AppResult},
    repository::sqlite::operational::room::room_repository::SqliteRoomRepository,
};

pub async fn get_room(db: &Db, input: GetRoomInput) -> AppResult<Room> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let room = SqliteRoomRepository::find_by_id(&mut tx, input.room_id)
            .await?
            .ok_or_else(|| not_found("room not found"))?;

        Ok(room)
    }
    .await;

    let _ = tx.rollback().await;

    result
}

use crate::{
    api::dto::input::room::UpdateRoomActivationInput,
    db::connection::Db,
    domain::entity::room::Room,
    error::app_error::{infra, not_found, AppResult},
    repository::sqlite::operational::room_repository::SqliteRoomRepository,
};

pub async fn update_room_activation(db: &Db, input: UpdateRoomActivationInput) -> AppResult<Room> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let mut room = SqliteRoomRepository::find_by_id(&mut tx, input.room_id)
            .await?
            .ok_or_else(|| not_found("room not found"))?;

        if input.is_active {
            room.activate();
        } else {
            room.deactivate();
        }

        SqliteRoomRepository::update(&mut tx, &room).await?;

        Ok(room)
    }
    .await;

    match result {
        Ok(room) => {
            tx.commit().await.map_err(infra)?;

            Ok(room)
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}

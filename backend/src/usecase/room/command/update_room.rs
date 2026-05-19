use crate::{
    api::dto::input::room::UpdateRoomInput,
    db::connection::Db,
    domain::entity::room::Room,
    error::app_error::{infra, not_found, AppResult},
    repository::sqlite::operational::room_repository::SqliteRoomRepository,
};

pub async fn update_room(db: &Db, input: UpdateRoomInput) -> AppResult<Room> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let mut room = SqliteRoomRepository::find_by_id(&mut tx, input.room_id)
            .await?
            .ok_or_else(|| not_found("room not found"))?;

        if let Some(room_no) = input.room_no {
            room.room_no = room_no;
        }

        if let Some(room_class) = input.room_class {
            room.room_class = room_class;
        }

        if let Some(capacity) = input.capacity {
            room.capacity = capacity;
        }

        if let Some(area_sqm) = input.area_sqm {
            room.area_sqm = area_sqm;
        }

        if let Some(is_physical) = input.is_physical {
            room.is_physical = is_physical;

            if !is_physical {
                room.capacity = None;
            }
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

use crate::db::connection::Db;

use crate::repository::sqlite::room_repository::SqliteRoomRepository;

pub async fn finish_cleaning(
    db: &Db,
    room_id: &str,
) -> Result<(), String> {

    let mut room =
        SqliteRoomRepository::find_by_id(
            &db.pool,
            room_id,
        )
        .await?
        .ok_or("room not found")?;

    room.finish_cleaning()?;

    SqliteRoomRepository::save(
        &db.pool,
        &room,
    )
    .await?;

    Ok(())
}
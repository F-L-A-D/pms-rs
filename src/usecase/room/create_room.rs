use crate::db::connection::Db;

use crate::domain::room::Room;

use crate::error::app_error::{
    AppError,
    AppResult,
};

use crate::repository::sqlite::room_repository::SqliteRoomRepository;

pub async fn create_room(
    db: &Db,
    room: Room,
) -> AppResult<()> {

    let mut tx =
        db.begin_tx().await;

    let result = async {

        SqliteRoomRepository::save(
            &mut tx,
            &room,
        )
        .await
        .map_err(AppError::Infrastructure)?;

        Ok(())

    }.await;

    match result {

        Ok(_) => {

            tx.commit()
                .await
                .map_err(|e| {
                    AppError::Infrastructure(
                        e.to_string()
                    )
                })?;

            Ok(())
        }

        Err(e) => {

            tx.rollback()
                .await
                .map_err(|e| {
                    AppError::Infrastructure(
                        e.to_string()
                    )
                })?;

            Err(e)
        }
    }
}
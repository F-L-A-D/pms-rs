use crate::db::connection::Db;

use crate::error::app_error::{
    AppError,
    AppResult,
};

use crate::repository::sqlite::room_repository::SqliteRoomRepository;

pub async fn mark_dirty(
    db: &Db,
    room_id: &str,
) -> AppResult<()> {

    let mut tx =
        db.begin_tx().await;

    let result = async {

        let mut room =
            SqliteRoomRepository::find_by_id(
                &mut tx,
                room_id,
            )
            .await
            .map_err(AppError::Infrastructure)?
            .ok_or(
                AppError::NotFound(
                    "room not found".into()
                )
            )?;

        room.mark_dirty()
            .map_err(AppError::Conflict)?;

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
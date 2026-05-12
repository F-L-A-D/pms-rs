use uuid::Uuid;

use crate::{
    db::connection::Db,

    error::app_error::{
        AppResult,
        conflict,
        infra,
        not_found,
    },

    repository::sqlite::operational::
        room_repository::
            SqliteRoomRepository,
};

pub async fn finish_cleaning(
    db: &Db,
    room_id: Uuid,
) -> AppResult<()> {

    let mut tx =
        db.begin_tx().await;

    let result = async {

        let mut room =
            SqliteRoomRepository
                ::find_by_id(
                    &mut tx,
                    room_id,
                )
                .await?
                .ok_or(
                    not_found(
                        "room not found"
                    )
                )?;

        room.finish_cleaning()
            .map_err(conflict)?;

        SqliteRoomRepository
            ::save(
                &mut tx,
                &room,
            )
            .await?;

        Ok(())

    }.await;

    match result {

        Ok(_) => {

            tx.commit()
                .await
                .map_err(infra)?;

            Ok(())
        }

        Err(e) => {

            let _ =
                tx.rollback().await;

            Err(e)
        }
    }
}
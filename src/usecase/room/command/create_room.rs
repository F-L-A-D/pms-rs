use crate::{
    db::connection::Db,

    domain::room::Room,

    error::app_error::{
        AppResult,
        infra,
    },

    repository::sqlite::operational::
        room_repository::SqliteRoomRepository,
};

pub async fn create_room(
    db: &Db,
    room: Room,
) -> AppResult<()> {

    let mut tx =
        db.begin_tx().await;

    let result = async {

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
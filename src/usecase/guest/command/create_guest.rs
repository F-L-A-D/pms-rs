use crate::{
    db::connection::Db,

    domain::guest::Guest,

    error::app_error::{
        AppResult,
        infra,
    },

    repository::sqlite::operational::
        guest_repository::
            SqliteGuestRepository,
};

pub async fn create_guest(
    db: &Db,
    guest: Guest,
) -> AppResult<()> {

    let mut tx =
        db.begin_tx().await;

    let result = async {

        SqliteGuestRepository
            ::save(
                &mut tx,
                &guest,
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
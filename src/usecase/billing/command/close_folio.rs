use uuid::Uuid;

use crate::{
    db::connection::Db,

    error::app_error::{
        AppResult,
        domain,
        infra,
        not_found,
    },

    repository::sqlite::operational::
        folio_repository::
            SqliteFolioRepository,
};

pub async fn close_folio(
    db: &Db,
    folio_id: Uuid,
) -> AppResult<()> {

    let mut tx =
        db.begin_tx().await;

    let result = async {

        let mut folio =
            SqliteFolioRepository
                ::find_by_id(
                    &mut tx,
                    folio_id,
                )
                .await?
                .ok_or(
                    not_found(
                        "folio not found"
                    )
                )?;

        folio.close()
            .map_err(domain)?;

        SqliteFolioRepository
            ::save(
                &mut tx,
                &folio,
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
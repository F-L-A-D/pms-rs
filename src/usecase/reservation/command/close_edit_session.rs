use crate::{
    api::dto::input::reservation::CloseReservationEditSessionInput,
    db::connection::Db,
    error::app_error::{infra, not_found, validation, AppResult},
    repository::sqlite::operational::reservation_edit_session_repository::SqliteReservationEditSessionRepository,
};

pub async fn execute(db: &Db, input: CloseReservationEditSessionInput) -> AppResult<()> {
    let mut tx = db.begin_tx().await;

    let result = async {
        if input.actor_id.trim().is_empty() {
            return Err(validation("actor_id is required"));
        }

        let closed = SqliteReservationEditSessionRepository::close(
            &mut tx,
            input.session_id,
            &input.actor_id,
            chrono::Utc::now(),
        )
        .await?;

        if !closed {
            return Err(not_found("reservation edit session not found"));
        }

        Ok(())
    }
    .await;

    match result {
        Ok(()) => {
            tx.commit().await.map_err(infra)?;

            Ok(())
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}

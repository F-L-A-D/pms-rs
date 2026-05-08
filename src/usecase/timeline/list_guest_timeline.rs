use uuid::Uuid;

use crate::db::connection::Db;

use crate::domain::guest_timeline_event::GuestTimelineEvent;

use crate::error::app_error::{
    AppError,
    AppResult,
};

use crate::repository::sqlite::operational::
    guest_timeline_event_repository::SqliteGuestTimelineEventRepository;

pub async fn list_guest_timeline(
    db: &Db,
    guest_id: Uuid,
) -> AppResult<Vec<GuestTimelineEvent>> {

    let mut tx =
        db.begin_tx().await;

    let result = async {

        let events =
            SqliteGuestTimelineEventRepository::find_by_guest_id(
                &mut tx,
                guest_id,
            )
            .await?;

        Ok(events)

    }.await;

    tx.rollback()
        .await
        .map_err(|e| {
            AppError::Infrastructure(
                e.to_string()
            )
        })?;

    result
}
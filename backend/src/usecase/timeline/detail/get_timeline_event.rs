use uuid::Uuid;

use crate::{
    db::connection::Db, domain::semantic::guest_timeline_event::GuestTimelineEvent,
    error::app_error::AppResult,
    repository::sqlite::operational::guest::guest_timeline_event_repository::SqliteGuestTimelineEventRepository,
};

pub async fn get_timeline_event(db: &Db, id: Uuid) -> AppResult<Option<GuestTimelineEvent>> {
    let mut tx = db.begin_tx().await;

    let event = SqliteGuestTimelineEventRepository::find_by_id(&mut tx, id).await?;

    let _ = tx.rollback().await;

    Ok(event)
}

use uuid::Uuid;

use crate::{
    db::connection::Db,

    domain::guest_timeline_event::
        GuestTimelineEvent,

    error::app_error::
        AppResult,

    repository::sqlite::operational::
        guest_timeline_event_repository::
            SqliteGuestTimelineEventRepository,
};

pub async fn get_guest_timelines(
    db: &Db,
    guest_id: Uuid,
) -> AppResult<Vec<GuestTimelineEvent>> {

    let mut tx =
        db.begin_tx().await;

    let events =
        SqliteGuestTimelineEventRepository
            ::find_by_guest_id(
                &mut tx,
                guest_id,
            )
            .await?;

    let _ =
        tx.rollback().await;

    Ok(events)
}
use sqlx::{
    Sqlite,
    Transaction,
};

use chrono::Utc;

use uuid::Uuid;

use crate::domain::guest_timeline_event::{
    GuestTimelineEvent,
    TimelineEventType,
};

use crate::error::app_error::AppResult;

use crate::repository::sqlite::operational::
    guest_timeline_event_repository::SqliteGuestTimelineEventRepository;

pub async fn record_event(
    tx: &mut Transaction<'_, Sqlite>,
    guest_id: Uuid,
    event_type: TimelineEventType,
    reference_id: String,
) -> AppResult<()> {

    let event =
        GuestTimelineEvent::new(
            format!(
                "timeline-{}",
                Utc::now().timestamp_millis()
            ),
            guest_id,
            event_type,
            reference_id,
        );

    SqliteGuestTimelineEventRepository::save(
        tx,
        &event,
    )
    .await?;

    Ok(())
}
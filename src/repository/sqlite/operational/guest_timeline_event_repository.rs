use chrono::{
    DateTime,
    Utc,
};

use sqlx::{
    Row,
    Sqlite,
    Transaction,
};

use uuid::Uuid;

use crate::domain::guest_timeline_event::{
    GuestTimelineEvent,
    TimelineEventType,
};

use crate::error::app_error::{
    AppError,
    AppResult,
};

pub struct SqliteGuestTimelineEventRepository;

impl SqliteGuestTimelineEventRepository {

    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        event: &GuestTimelineEvent,
    ) -> AppResult<()> {

        sqlx::query(
            r#"
            INSERT INTO guest_timeline_events (
                id,
                guest_id,
                event_type,
                reference_id,
                occurred_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#
        )
        .bind(&event.id)
        .bind(&event.guest_id.to_string())
        .bind(format!("{:?}", event.event_type))
        .bind(&event.reference_id)
        .bind(event.occurred_at.to_rfc3339())
        .execute(&mut **tx)
        .await
        .map_err(|e| {
            AppError::Infrastructure(
                e.to_string()
            )
        })?;

        Ok(())
    }

    pub async fn find_by_guest_id(
        tx: &mut Transaction<'_, Sqlite>,
        guest_id: Uuid,
    ) -> AppResult<Vec<GuestTimelineEvent>> {

        let rows =
            sqlx::query(
                r#"
                SELECT
                    id,
                    guest_id,
                    event_type,
                    reference_id,
                    occurred_at
                FROM guest_timeline_events
                WHERE guest_id = ?1
                ORDER BY occurred_at DESC
                "#
            )
            .bind(guest_id.to_string())
            .fetch_all(&mut **tx)
            .await
            .map_err(|e| {
            AppError::Infrastructure(
                e.to_string()
            )
        })?;

        let mut events = vec![];

        for r in rows {

            let event_type =
                match r.get::<String, _>("event_type").as_str() {

                    "ReservationCreated" =>
                        TimelineEventType::ReservationCreated,

                    "ReservationCancelled" =>
                        TimelineEventType::ReservationCancelled,

                    "CheckedIn" =>
                        TimelineEventType::CheckedIn,

                    "CheckedOut" =>
                        TimelineEventType::CheckedOut,

                    "RoomChargePosted" =>
                        TimelineEventType::RoomChargePosted,

                    _ => {
                        return Err(
                            AppError::Validation(
                                "invalid timeline event".into()
                            )
                        )
                    }
                };

            let occurred_at =
                DateTime::parse_from_rfc3339(
                    &r.get::<String, _>("occurred_at")
                )
                .map_err(|e| {
                    AppError::Infrastructure(
                        e.to_string()
                    )
                })?
                .with_timezone(&Utc);

            events.push(
                GuestTimelineEvent {
                    id: r.get("id"),

                    guest_id:
                        Uuid::parse_str(
                        r.get::<String, _>("guest_id")
                            .as_str()
                        )
                        .unwrap(),

                    event_type,

                    reference_id:
                        r.get("reference_id"),

                    occurred_at,
                }
            );
        }

        Ok(events)
    }
}
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

use crate::{
    domain::guest_timeline_event::{
        GuestTimelineEvent,
        TimelineEventType,
    },

    error::app_error::{
        AppResult,
        infra,
    },
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
        .bind(event.guest_id.to_string())
        .bind(
            match event.event_type {

                TimelineEventType::ReservationCreated =>
                    "ReservationCreated",

                TimelineEventType::ReservationCancelled =>
                    "ReservationCancelled",

                TimelineEventType::CheckedIn =>
                    "CheckedIn",

                TimelineEventType::CheckedOut =>
                    "CheckedOut",

                TimelineEventType::RoomChargePosted =>
                    "RoomChargePosted",
            }
        )
        .bind(&event.reference_id)
        .bind(event.occurred_at.to_rfc3339())
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn find_by_id(
        tx: &mut Transaction<'_, Sqlite>,
        id: Uuid,
    ) -> AppResult<Option<GuestTimelineEvent>> {

        let row =
            sqlx::query(
                r#"
                SELECT
                    id,
                    guest_id,
                    event_type,
                    reference_id,
                    occurred_at
                FROM guest_timeline_events
                WHERE id = ?1
                ORDER BY occurred_at DESC
                "#
            )
            .bind(id.to_string())
            .fetch_optional(&mut **tx)
            .await
            .map_err(infra)?;

        match row {

            Some(r) => {
                Ok(
                    Some(
                        Self::row_to_event(&r)?
                    )
                )
            }

            None => Ok(None),
        }
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
            .map_err(infra)?;

        Ok(
            rows.iter()
                .map(Self::row_to_event)
                .collect::<AppResult<Vec<_>>>()?
        )
    }

    fn row_to_event(
        row: &sqlx::sqlite::SqliteRow,
    ) -> AppResult<GuestTimelineEvent> {

        let event_type =
            match row
                .get::<String, _>("event_type")
                .as_str()
            {

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
                        infra(
                            "invalid timeline event"
                        )
                    )
                }
            };

        let occurred_at =
            DateTime::parse_from_rfc3339(
                row.get::<String, _>("occurred_at")
                    .as_str()
            )
            .map_err(infra)?
            .with_timezone(&Utc);

        Ok(
            GuestTimelineEvent {

                id:
                    row.get("id"),

                guest_id:
                    Uuid::parse_str(
                        row.get::<String, _>("guest_id")
                            .as_str()
                    )
                    .map_err(infra)?,

                event_type,

                reference_id:
                    row.get("reference_id"),

                occurred_at,
            }
        )
    }
}
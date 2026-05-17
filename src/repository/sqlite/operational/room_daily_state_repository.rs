use chrono::{DateTime, NaiveDate, Utc};

use sqlx::{Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::semantic::room_daily_state::{
        RoomDailyHousekeepingStatus, RoomDailyOccupancyStatus, RoomDailyState,
    },
    error::app_error::{infra, AppResult},
};

pub struct SqliteRoomDailyStateRepository;

impl SqliteRoomDailyStateRepository {
    pub async fn save(tx: &mut Transaction<'_, Sqlite>, state: &RoomDailyState) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO room_daily_states (
                room_id,
                service_date,
                occupancy_status,
                housekeeping_status,
                updated_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
        )
        .bind(state.room_id.to_string())
        .bind(state.service_date.to_string())
        .bind(state.occupancy_status.to_snake())
        .bind(state.housekeeping_status.to_snake())
        .bind(state.updated_at.to_rfc3339())
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn find_by_room_and_service_date(
        tx: &mut Transaction<'_, Sqlite>,
        room_id: Uuid,
        service_date: NaiveDate,
    ) -> AppResult<Option<RoomDailyState>> {
        let row = sqlx::query(
            r#"
            SELECT
                room_id,
                service_date,
                occupancy_status,
                housekeeping_status,
                updated_at
            FROM room_daily_states
            WHERE room_id = ?1
                AND service_date = ?2
            "#,
        )
        .bind(room_id.to_string())
        .bind(service_date.to_string())
        .fetch_optional(&mut **tx)
        .await
        .map_err(infra)?;

        match row {
            Some(row) => Ok(Some(Self::row_to_room_daily_state(&row)?)),
            None => Ok(None),
        }
    }

    pub async fn list_by_service_date(
        tx: &mut Transaction<'_, Sqlite>,
        service_date: NaiveDate,
    ) -> AppResult<Vec<RoomDailyState>> {
        let rows = sqlx::query(
            r#"
            SELECT
                room_id,
                service_date,
                occupancy_status,
                housekeeping_status,
                updated_at
            FROM room_daily_states
            WHERE service_date = ?1
            ORDER BY room_id
            "#,
        )
        .bind(service_date.to_string())
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

        rows.iter().map(Self::row_to_room_daily_state).collect()
    }

    fn row_to_room_daily_state(row: &sqlx::sqlite::SqliteRow) -> AppResult<RoomDailyState> {
        let occupancy_status =
            RoomDailyOccupancyStatus::from_snake(row.get::<String, _>("occupancy_status").as_str())
                .ok_or_else(|| infra("invalid room daily occupancy status"))?;

        let housekeeping_status = RoomDailyHousekeepingStatus::from_snake(
            row.get::<String, _>("housekeeping_status").as_str(),
        )
        .ok_or_else(|| infra("invalid room daily housekeeping status"))?;

        let updated_at = DateTime::parse_from_rfc3339(row.get::<String, _>("updated_at").as_str())
            .map_err(infra)?
            .with_timezone(&Utc);

        Ok(RoomDailyState {
            room_id: Uuid::parse_str(row.get::<String, _>("room_id").as_str()).map_err(infra)?,
            service_date: NaiveDate::parse_from_str(
                row.get::<String, _>("service_date").as_str(),
                "%Y-%m-%d",
            )
            .map_err(infra)?,
            occupancy_status,
            housekeeping_status,
            updated_at,
        })
    }
}

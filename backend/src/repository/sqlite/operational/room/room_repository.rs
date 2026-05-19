use rust_decimal::Decimal;
use sqlx::{Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::entity::room::Room,
    error::app_error::{infra, AppResult},
};

pub struct SqliteRoomRepository;

impl SqliteRoomRepository {
    pub async fn save(tx: &mut Transaction<'_, Sqlite>, room: &Room) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO rooms (
                id,
                room_no,
                room_class,
                capacity,
                area_sqm,
                is_physical,
                is_active
            )
            VALUES (
                ?, ?, ?, ?, ?, ?, ?
            )
            "#,
        )
        .bind(room.id.to_string())
        .bind(&room.room_no)
        .bind(&room.room_class)
        .bind(room.capacity.map(|v| v as i64))
        .bind(room.area_sqm.to_string())
        .bind(room.is_physical)
        .bind(room.is_active)
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn update(tx: &mut Transaction<'_, Sqlite>, room: &Room) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE rooms
            SET
                room_no = ?,
                room_class = ?,
                capacity = ?,
                area_sqm = ?,
                is_physical = ?,
                is_active = ?
            WHERE id = ?
            "#,
        )
        .bind(&room.room_no)
        .bind(&room.room_class)
        .bind(room.capacity.map(|v| v as i64))
        .bind(room.area_sqm.to_string())
        .bind(room.is_physical)
        .bind(room.is_active)
        .bind(room.id.to_string())
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    fn map_room(row: sqlx::sqlite::SqliteRow) -> AppResult<Room> {
        let area_sqm = row
            .get::<String, _>("area_sqm")
            .parse::<Decimal>()
            .map_err(infra)?;

        Ok(Room {
            id: Uuid::parse_str(&row.get::<String, _>("id")).map_err(infra)?,

            room_no: row.get("room_no"),

            room_class: row.get("room_class"),

            capacity: row.get::<Option<i64>, _>("capacity").map(|v| v as u32),

            area_sqm,

            is_physical: row.get("is_physical"),

            is_active: row.get("is_active"),
        })
    }

    pub async fn find_by_id(
        tx: &mut Transaction<'_, Sqlite>,
        room_id: Uuid,
    ) -> AppResult<Option<Room>> {
        let row = sqlx::query(
            r#"
                SELECT *
                FROM rooms
                WHERE id = ?
                "#,
        )
        .bind(room_id.to_string())
        .fetch_optional(&mut **tx)
        .await
        .map_err(infra)?;

        match row {
            Some(row) => Ok(Some(Self::map_room(row)?)),

            None => Ok(None),
        }
    }

    pub async fn find_all(tx: &mut Transaction<'_, Sqlite>) -> AppResult<Vec<Room>> {
        let rows = sqlx::query(
            r#"
                SELECT *
                FROM rooms
                ORDER BY room_no
                "#,
        )
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

        rows.into_iter().map(Self::map_room).collect()
    }

    pub async fn find_active(tx: &mut Transaction<'_, Sqlite>) -> AppResult<Vec<Room>> {
        let rows = sqlx::query(
            r#"
                SELECT *
                FROM rooms
                WHERE is_active = 1
                ORDER BY room_no
                "#,
        )
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

        rows.into_iter().map(Self::map_room).collect()
    }
}

use sqlx::{
    Row,
    Sqlite,
    Transaction,
};

use uuid::Uuid;

use crate::{
    error::app_error::{
        AppResult,
        infra,
    },

    domain::room::{
        Room,
        OccupancyStatus,
        HousekeepingStatus,
    },
};

pub struct SqliteRoomRepository;

impl SqliteRoomRepository {

    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        room: &Room,
    ) -> AppResult<()> {

        sqlx::query(
            r#"
            INSERT OR REPLACE INTO rooms (
                id,
                room_class,
                occupancy_status,
                housekeeping_status
            )
            VALUES (?, ?, ?, ?)
            "#
        )
        .bind(&room.id)
        .bind(&room.room_class)
        .bind(match room.occupancy_status {
            OccupancyStatus::Occupied =>
                "Occupied",

            OccupancyStatus::Vacant =>
                "Vacant",
        })
        .bind(match room.housekeeping_status {
            HousekeepingStatus::Dirty =>
                "Dirty",

            HousekeepingStatus::Cleaning =>
                "Cleaning",

            HousekeepingStatus::Cleaned =>
                "Cleaned",

            HousekeepingStatus::Inspected =>
                "Inspected",
        })
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn find_by_id(
        tx: &mut Transaction<'_, Sqlite>,
        id: Uuid,
    ) -> AppResult<Option<Room>> {

        let row =
            sqlx::query(
                r#"
                SELECT
                    id,
                    room_no,
                    room_class,
                    occupancy_status,
                    housekeeping_status
                FROM rooms
                WHERE id = ?
                "#
            )
            .bind(id.to_string())
            .fetch_optional(&mut **tx)
            .await
            .map_err(infra)?;

        match row {

            Some(row) => {
                Ok(
                    Some(
                        Self::row_to_room(&row)?
                    )
                )
            }

            None => Ok(None),
        }
    }

    pub async fn find_by_room_class(
        tx: &mut Transaction<'_, Sqlite>,
        room_class: &str,
    ) -> AppResult<Vec<Room>> {

        let rows =
            sqlx::query(
                r#"
                SELECT
                    id,
                    room_no,
                    room_class,
                    occupancy_status,
                    housekeeping_status
                FROM rooms
                WHERE room_class = ?
                "#
            )
            .bind(room_class)
            .fetch_all(&mut **tx)
            .await
            .map_err(infra)?;

        Ok(
            rows.iter()
                .map(Self::row_to_room)
                .collect::<AppResult<Vec<_>>>()?
        )
    }

    pub async fn find_all(
        tx: &mut Transaction<'_, Sqlite>,
    ) -> AppResult<Vec<Room>> {

        let rows =
            sqlx::query(
                r#"
                SELECT
                    id,
                    room_no,
                    room_class,
                    occupancy_status,
                    housekeeping_status
                FROM rooms
                "#
            )
            .fetch_all(&mut **tx)
            .await
            .map_err(infra)?;

        Ok(
            rows.iter()
                .map(Self::row_to_room)
                .collect::<AppResult<Vec<_>>>()?
        )
    }

    fn row_to_room(
        row: &sqlx::sqlite::SqliteRow,
    ) -> AppResult<Room> {

        let occupancy_status =
            match row
                .get::<String, _>("occupancy_status")
                .as_str()
            {
                "Occupied" =>
                    OccupancyStatus::Occupied,

                "Vacant" =>
                    OccupancyStatus::Vacant,

                _ =>
                    return Err(
                        infra(
                            "invalid occupancy status"
                        )
                    ),
            };

        let housekeeping_status =
            match row
                .get::<String, _>("housekeeping_status")
                .as_str()
            {
                "Dirty" =>
                    HousekeepingStatus::Dirty,

                "Cleaning" =>
                    HousekeepingStatus::Cleaning,

                "Cleaned" =>
                    HousekeepingStatus::Cleaned,

                "Inspected" =>
                    HousekeepingStatus::Inspected,

                _ =>
                    return Err(
                        infra(
                            "invalid housekeeping status"
                        )
                    ),
            };

        Ok(
            Room {
                id:
                    Uuid::parse_str(
                        row.get::<String, _>("id")
                            .as_str()
                    )
                    .map_err(infra)?,

                room_no:
                    row.get("room_no"),

                room_class:
                    row.get("room_class"),

                occupancy_status,

                housekeeping_status,
            }
        )
    }
}
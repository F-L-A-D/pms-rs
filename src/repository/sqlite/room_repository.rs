use sqlx::{Row, Sqlite, Transaction};

use crate::domain::room::{
    Room,
    OccupancyStatus,
    HousekeepingStatus,
};

pub struct SqliteRoomRepository;

impl SqliteRoomRepository {

    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        room: &Room,
    ) -> Result<(), String> {

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
        .bind(format!("{:?}", room.occupancy_status))
        .bind(format!("{:?}", room.housekeeping_status))
        .execute(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn find_by_id(
        tx: &mut Transaction<'_, Sqlite>,
        id: &str,
    ) -> Result<Option<Room>, String> {

        let row =
            sqlx::query(
                r#"
                SELECT
                    id,
                    room_class,
                    occupancy_status,
                    housekeeping_status
                FROM rooms
                WHERE id = ?
                "#
            )
            .bind(id)
            .fetch_optional(&mut **tx)
            .await
            .map_err(|e| e.to_string())?;

        match row {

            Some(row) => {

                let occupancy_status =
                    match row.get::<String, _>("occupancy_status").as_str() {
                        "Occupied" => OccupancyStatus::Occupied,
                        _ => OccupancyStatus::Vacant,
                    };

                let housekeeping_status =
                    match row.get::<String, _>("housekeeping_status").as_str() {
                        "Dirty" => HousekeepingStatus::Dirty,
                        "Cleaning" => HousekeepingStatus::Cleaning,
                        "Cleaned" => HousekeepingStatus::Cleaned,
                        _ => HousekeepingStatus::Inspected,
                    };

                Ok(
                    Some(
                        Room {
                            id: row.get("id"),
                            room_class: row.get("room_class"),
                            occupancy_status,
                            housekeeping_status,
                        }
                    )
                )
            }

            None => Ok(None),
        }
    }

    pub async fn find_all(
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<Vec<Room>, String> {

        let rows =
            sqlx::query(
                r#"
                SELECT
                    id,
                    room_class,
                    occupancy_status,
                    housekeeping_status
                FROM rooms
                "#
            )
            .fetch_all(&mut **tx)
            .await
            .map_err(|e| e.to_string())?;

        let mut rooms = vec![];

        for row in rows {

            let occupancy_status =
                match row.get::<String, _>("occupancy_status").as_str() {
                    "Occupied" => OccupancyStatus::Occupied,
                    _ => OccupancyStatus::Vacant,
                };

            let housekeeping_status =
                match row.get::<String, _>("housekeeping_status").as_str() {
                    "Dirty" => HousekeepingStatus::Dirty,
                    "Cleaning" => HousekeepingStatus::Cleaning,
                    "Cleaned" => HousekeepingStatus::Cleaned,
                    _ => HousekeepingStatus::Inspected,
                };

            rooms.push(
                Room {
                    id: row.get("id"),
                    room_class: row.get("room_class"),
                    occupancy_status,
                    housekeeping_status,
                }
            );
        }

        Ok(rooms)
    }
}
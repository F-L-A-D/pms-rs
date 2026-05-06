use sqlx::Row;

use crate::domain::room::{
    Room,
    OccupancyStatus,
    HousekeepingStatus,
};

pub struct SqliteRoomRepository;

impl SqliteRoomRepository {

    pub async fn save(
        db: &sqlx::SqlitePool,
        room: &Room,
    ) -> Result<(), String> {

        sqlx::query(
            r#"
            INSERT INTO rooms (id, room_type, occupancy_status, housekeeping_status)
            VALUES (?1, ?2, ?3, ?4)
            "#
        )
        .bind(&room.id)
        .bind(&room.room_type)
        .bind(format!("{:?}", room.occupancy_status))
        .bind(format!("{:?}", room.housekeeping_status))
        .execute(db)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn find_by_id(
        db: &sqlx::SqlitePool,
        id: &str,
    ) -> Result<Option<Room>, String> {

        let row = sqlx::query(
            r#"
            SELECT id, room_type, occupancy_status, housekeeping_status
            FROM rooms
            WHERE id = ?1
            "#
        )
        .bind(id)
        .fetch_optional(db)
        .await
        .map_err(|e| e.to_string())?;

        if let Some(r) = row {

            let occ_str: String = r.get("occupancy_status");
            let hk_str: String = r.get("housekeeping_status");

            let occupancy_status = match occ_str.as_str() {
                "Vacant" => OccupancyStatus::Vacant,
                "Occupied" => OccupancyStatus::Occupied,
                _ => return Err("invalid occupancy status".into()),
            };

            let housekeeping_status = match hk_str.as_str() {
                "Dirty" => HousekeepingStatus::Dirty,
                "Cleaning" => HousekeepingStatus::Cleaning,
                "Cleaned" => HousekeepingStatus::Cleaned,
                "Inspected" => HousekeepingStatus::Inspected,
                _ => return Err("invalid housekeeping status".into()),
            };

            Ok(Some(Room {
                id: r.get("id"),
                room_type: r.get("room_type"),
                occupancy_status,
                housekeeping_status,
            }))
        } else {
            Ok(None)
        }
    }
}
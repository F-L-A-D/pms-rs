#![allow(unused_imports)]
use pms_rs::db::connection::Db;
use pms_rs::repository::sqlite::room_repository::SqliteRoomRepository;
use pms_rs::domain::room::{Room, OccupancyStatus, HousekeepingStatus};

#[tokio::test]
async fn should_create_and_find_room() {
    let db = Db::new("sqlite::memory:").await;

    let room = Room {
        id: "101".into(),
        room_class: "single".into(),
        occupancy_status: OccupancyStatus::Vacant,
        housekeeping_status: HousekeepingStatus::Inspected,
    };

    SqliteRoomRepository::save(&db.pool, &room)
        .await
        .unwrap();

    let fetched = SqliteRoomRepository::find_by_id(&db.pool, "101")
        .await
        .unwrap()
        .unwrap();

    assert_eq!(fetched.occupancy_status, OccupancyStatus::Vacant);
    assert_eq!(fetched.housekeeping_status, HousekeepingStatus::Inspected);
}
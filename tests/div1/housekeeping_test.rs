#![allow(unused_imports)]
use pms_rs::db::connection::Db;

use pms_rs::domain::room::{
    Room,
    OccupancyStatus,
    HousekeepingStatus,
};

use pms_rs::repository::sqlite::room_repository::SqliteRoomRepository;

use pms_rs::usecase::housekeeping::mark_dirty::mark_dirty;
use pms_rs::usecase::housekeeping::start_cleaning::start_cleaning;
use pms_rs::usecase::housekeeping::finish_cleaning::finish_cleaning;
use pms_rs::usecase::housekeeping::inspect_room::inspect_room;

#[tokio::test]
async fn should_complete_housekeeping_flow() {

    let db = Db::new("sqlite::memory:").await;

    let room = Room::new(
        "101".into(),
        "single".into(),
    );

    SqliteRoomRepository::save(&db.pool, &room)
        .await
        .unwrap();

    mark_dirty(&db, "101")
        .await
        .unwrap();

    start_cleaning(&db, "101")
        .await
        .unwrap();

    finish_cleaning(&db, "101")
        .await
        .unwrap();

    let result = inspect_room(&db, "101").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn should_fail_start_cleaning_when_not_dirty() {

    let db = Db::new("sqlite::memory:").await;

    let room = Room::new (
        "101".into(),
        "single".into(),
    );

    SqliteRoomRepository::save(&db.pool, &room)
        .await
        .unwrap();

    let result = start_cleaning(&db, "101").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn should_fail_finish_cleaning_when_not_cleaning() {

    let db = Db::new("sqlite::memory:").await;

    let room = Room::new(
        "101".into(),
        "single".into(),
    );

    SqliteRoomRepository::save(&db.pool, &room)
        .await
        .unwrap();

    mark_dirty(&db, "101")
        .await
        .unwrap();
    
    let result = finish_cleaning(&db, "101").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn should_fail_inspect_when_not_cleaned() {

    let db = Db::new("sqlite::memory:").await;

    let room = Room {
        id: "101".into(),
        room_class: "single".into(),
        occupancy_status: OccupancyStatus::Vacant,
        housekeeping_status: HousekeepingStatus::Cleaning,
    };

    SqliteRoomRepository::save(&db.pool, &room)
        .await
        .unwrap();

    let result = inspect_room(&db, "101").await;

    assert!(result.is_err());
}
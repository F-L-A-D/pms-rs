#![allow(unused_imports)]
use chrono::NaiveDate;

use pms_rs::db::connection::Db;

use pms_rs::adapter::stay_input::StayInput;

use pms_rs::domain::room::{
    Room,
    OccupancyStatus,
    HousekeepingStatus,
};

use pms_rs::repository::sqlite::room_repository::SqliteRoomRepository;

use pms_rs::usecase::reservation::create::create;
use pms_rs::usecase::reservation::assign_room::assign_room;

use pms_rs::usecase::stay::check_in::check_in;

#[tokio::test]
async fn should_check_in() {

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

    create(
        &db,
        "r1".into(),
        StayInput::CheckInAndNights {
            check_in: NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
            nights: 1,
            room_class: "single".into(),
        },
    )
    .await
    .unwrap();

    assign_room(&db, "r1", "101")
        .await
        .unwrap();

    let result = check_in(&db, "r1").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn should_fail_double_checkin() {

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

    create(
        &db,
        "r1".into(),
        StayInput::CheckInAndNights {
            check_in: NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
            nights: 1,
            room_class: "single".into(),
        },
    )
    .await
    .unwrap();

    assign_room(&db, "r1", "101")
        .await
        .unwrap();

    check_in(&db, "r1")
        .await
        .unwrap();

    let result = check_in(&db, "r1").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn should_fail_checkin_when_room_dirty() {

    let db = Db::new("sqlite::memory:").await;

    let room = Room {
        id: "101".into(),
        room_class: "single".into(),
        occupancy_status: OccupancyStatus::Vacant,
        housekeeping_status: HousekeepingStatus::Dirty,
    };

    SqliteRoomRepository::save(&db.pool, &room)
        .await
        .unwrap();

    create(
        &db,
        "r1".into(),
        StayInput::CheckInAndNights {
            check_in: NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
            nights: 1,
            room_class: "single".into(),
        },
    )
    .await
    .unwrap();

    assign_room(&db, "r1", "101")
        .await
        .unwrap();

    let result = check_in(&db, "r1").await;

    assert!(result.is_err());
}
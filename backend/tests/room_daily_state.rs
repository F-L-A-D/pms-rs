use chrono::NaiveDate;

use rust_decimal::Decimal;

use uuid::Uuid;

use pms_rs::{
    db::connection::Db,
    domain::{
        entity::room::Room,
        semantic::room_daily_state::{
            RoomDailyHousekeepingStatus, RoomDailyOccupancyStatus, RoomDailyState,
        },
    },
    repository::sqlite::operational::room::{
        room_daily_state_repository::SqliteRoomDailyStateRepository,
        room_repository::SqliteRoomRepository,
    },
};

#[tokio::test]
async fn should_save_and_find_room_daily_state() {
    let db = Db::new_test().await;
    let room_id = Uuid::new_v4();
    let service_date = NaiveDate::from_ymd_opt(2026, 5, 17).unwrap();

    seed_room(&db, room_id).await;

    let mut state = RoomDailyState::new(room_id, service_date);
    state.set_occupancy_status(RoomDailyOccupancyStatus::Occupied);
    state.mark_dirty();

    let mut tx = db.begin_tx().await;

    SqliteRoomDailyStateRepository::save(&mut tx, &state)
        .await
        .unwrap();

    let persisted = SqliteRoomDailyStateRepository::find_by_room_and_service_date(
        &mut tx,
        room_id,
        service_date,
    )
    .await
    .unwrap()
    .unwrap();

    assert_eq!(persisted.room_id, room_id);
    assert_eq!(persisted.service_date, service_date);
    assert_eq!(
        persisted.occupancy_status,
        RoomDailyOccupancyStatus::Occupied,
    );
    assert_eq!(
        persisted.housekeeping_status,
        RoomDailyHousekeepingStatus::Dirty,
    );

    let _ = tx.rollback().await;
}

#[tokio::test]
async fn should_return_none_when_room_daily_state_does_not_exist() {
    let db = Db::new_test().await;
    let room_id = Uuid::new_v4();
    let service_date = NaiveDate::from_ymd_opt(2026, 5, 17).unwrap();
    let mut tx = db.begin_tx().await;

    let result = SqliteRoomDailyStateRepository::find_by_room_and_service_date(
        &mut tx,
        room_id,
        service_date,
    )
    .await
    .unwrap();

    assert!(result.is_none());

    let _ = tx.rollback().await;
}

#[tokio::test]
async fn should_list_room_daily_states_by_service_date() {
    let db = Db::new_test().await;
    let service_date = NaiveDate::from_ymd_opt(2026, 5, 17).unwrap();
    let other_date = NaiveDate::from_ymd_opt(2026, 5, 18).unwrap();
    let room_id = Uuid::new_v4();
    let other_room_id = Uuid::new_v4();

    seed_room(&db, room_id).await;
    seed_room(&db, other_room_id).await;

    let mut tx = db.begin_tx().await;

    SqliteRoomDailyStateRepository::save(&mut tx, &RoomDailyState::new(room_id, service_date))
        .await
        .unwrap();

    SqliteRoomDailyStateRepository::save(&mut tx, &RoomDailyState::new(other_room_id, other_date))
        .await
        .unwrap();

    let states = SqliteRoomDailyStateRepository::list_by_service_date(&mut tx, service_date)
        .await
        .unwrap();

    assert_eq!(states.len(), 1);
    assert_eq!(states[0].room_id, room_id);

    let _ = tx.rollback().await;
}

async fn seed_room(db: &Db, room_id: Uuid) {
    let room = Room {
        id: room_id,
        room_no: format!("R-{}", &room_id.to_string()[..8]),
        room_class: "standard".to_string(),
        capacity: Some(2),
        area_sqm: Decimal::new(2400, 2),
        is_physical: true,
        is_active: true,
    };

    let mut tx = db.begin_tx().await;

    SqliteRoomRepository::save(&mut tx, &room).await.unwrap();

    tx.commit().await.unwrap();
}

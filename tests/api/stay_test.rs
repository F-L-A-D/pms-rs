use axum::http::StatusCode;

use pms_rs::{
    domain::semantic::room_daily_state::{
        RoomDailyHousekeepingStatus, RoomDailyOccupancyStatus, RoomDailyState,
    },
    repository::sqlite::operational::room_daily_state_repository::SqliteRoomDailyStateRepository,
};

use crate::common::{
    app::{spawn_app, TestApp},
    client::post,
    reservation::create_reservation,
    room::create_room,
    stay::{assign_room, check_in, check_out},
};

#[tokio::test]
async fn should_chek_in_reservation() {
    let app = spawn_app().await;

    let reservation = create_reservation(&app.app).await;

    let room = create_room(&app.app).await;

    assign_room(&app.app, reservation.id, room.id).await;

    let response = check_in(&app.app, reservation.id).await;

    assert_eq!(response.id, reservation.id,);

    assert_eq!(response.status, "checked_in",);

    let room_state = find_room_daily_state(&app, room.id, reservation.check_in).await;

    assert_eq!(
        room_state.occupancy_status,
        RoomDailyOccupancyStatus::Occupied,
    );
}

#[tokio::test]
async fn should_chek_out_reservation() {
    let app = spawn_app().await;

    let reservation = create_reservation(&app.app).await;

    let room = create_room(&app.app).await;

    assign_room(&app.app, reservation.id, room.id).await;

    check_in(&app.app, reservation.id).await;

    let response = check_out(&app.app, reservation.id).await;

    assert_eq!(response.id, reservation.id,);

    assert_eq!(response.status, "checked_out",);

    let room_state = find_room_daily_state(&app, room.id, reservation.check_out).await;

    assert_eq!(
        room_state.occupancy_status,
        RoomDailyOccupancyStatus::Vacant,
    );

    assert_eq!(
        room_state.housekeeping_status,
        RoomDailyHousekeepingStatus::Dirty,
    );
}

#[tokio::test]
async fn should_reject_check_in_when_room_is_out_of_order() {
    let app = spawn_app().await;

    let reservation = create_reservation(&app.app).await;

    let room = create_room(&app.app).await;

    assign_room(&app.app, reservation.id, room.id).await;

    let mut room_state = RoomDailyState::new(room.id, reservation.check_in);

    room_state.set_occupancy_status(RoomDailyOccupancyStatus::OutOfOrder);

    let mut tx = app.db.begin_tx().await;

    SqliteRoomDailyStateRepository::save(&mut tx, &room_state)
        .await
        .unwrap();

    tx.commit().await.unwrap();

    let response = post(
        &app.app,
        &format!("/reservations/{}/check-in", reservation.id,),
    )
    .await;

    assert_eq!(response.status(), StatusCode::CONFLICT,);
}

async fn find_room_daily_state(
    app: &TestApp,
    room_id: uuid::Uuid,
    service_date: chrono::NaiveDate,
) -> RoomDailyState {
    let mut tx = app.db.begin_tx().await;

    let state = SqliteRoomDailyStateRepository::find_by_room_and_service_date(
        &mut tx,
        room_id,
        service_date,
    )
    .await
    .unwrap()
    .unwrap();

    let _ = tx.rollback().await;

    state
}

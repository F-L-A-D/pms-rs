use axum::http::StatusCode;

use chrono::{Duration, Utc};

use pms_rs::{
    api::dto::reservation::ReservationResponse,
    domain::semantic::reservation_transition::ReservationTransitionType,
    domain::semantic::room_daily_state::{
        RoomDailyHousekeepingStatus, RoomDailyOccupancyStatus, RoomDailyState,
    },
    repository::sqlite::{
        behavioral::reservation_transition_repository::SqliteReservationTransitionRepository,
        operational::room_daily_state_repository::SqliteRoomDailyStateRepository,
    },
};

use crate::common::{
    app::{spawn_app, TestApp},
    builders::{ReservationBuilder, ReservationParticipantBuilder},
    client::{get, post, post_json, response_json},
    guest::create_guest,
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

#[tokio::test]
async fn should_move_checked_in_reservation_to_another_room() {
    let app = spawn_app().await;

    let today = Utc::now().date_naive();
    let guest = create_guest(&app.app).await;
    let participant = ReservationParticipantBuilder::new(guest.id).build();
    let request = ReservationBuilder::new()
        .with_participant(participant)
        .with_check_in(today)
        .with_check_out(today + Duration::days(2))
        .build();

    let create_response = post_json(&app.app, "/reservations", &request).await;

    assert_eq!(create_response.status(), StatusCode::CREATED,);

    let reservation: ReservationResponse =
        serde_json::from_value(response_json(create_response).await).unwrap();

    let old_room = create_room(&app.app).await;
    let new_room = create_room(&app.app).await;

    assign_room(&app.app, reservation.id, old_room.id).await;
    check_in(&app.app, reservation.id).await;

    let move_response = post_json(
        &app.app,
        &format!("/reservations/{}/room-move/{}", reservation.id, new_room.id),
        &serde_json::json!({ "effective_date": today }),
    )
    .await;

    assert_eq!(move_response.status(), StatusCode::OK,);

    let body = response_json(move_response).await;

    assert_eq!(body["status"], "room_moved");

    let old_room_state = find_room_daily_state(&app, old_room.id, today).await;
    let new_room_state = find_room_daily_state(&app, new_room.id, today).await;
    let next_new_room_state =
        find_room_daily_state(&app, new_room.id, today + Duration::days(1)).await;

    assert_eq!(
        old_room_state.occupancy_status,
        RoomDailyOccupancyStatus::Vacant,
    );
    assert_eq!(
        old_room_state.housekeeping_status,
        RoomDailyHousekeepingStatus::Dirty,
    );
    assert_eq!(
        new_room_state.occupancy_status,
        RoomDailyOccupancyStatus::Occupied,
    );
    assert_eq!(
        next_new_room_state.occupancy_status,
        RoomDailyOccupancyStatus::Occupied,
    );

    let get_response = get(&app.app, &format!("/reservations/{}", reservation.id)).await;

    assert_eq!(get_response.status(), StatusCode::OK,);

    let updated: ReservationResponse =
        serde_json::from_value(response_json(get_response).await).unwrap();

    assert_eq!(updated.room_id, Some(new_room.id));

    let mut tx = app.db.begin_tx().await;

    let transitions =
        SqliteReservationTransitionRepository::find_by_reservation_id(&mut tx, reservation.id)
            .await
            .unwrap();

    let _ = tx.rollback().await;

    assert!(transitions.iter().any(|transition| {
        transition.transition_type == ReservationTransitionType::RoomMoved
            && transition.field_name == "room_id"
            && transition.before_value == old_room.id.to_string()
            && transition.after_value == new_room.id.to_string()
    }));
}

#[tokio::test]
async fn should_reject_room_move_when_target_room_is_occupied() {
    let app = spawn_app().await;

    let reservation = create_reservation(&app.app).await;
    let old_room = create_room(&app.app).await;
    let new_room = create_room(&app.app).await;

    assign_room(&app.app, reservation.id, old_room.id).await;
    check_in(&app.app, reservation.id).await;

    let mut new_room_state = RoomDailyState::new(new_room.id, reservation.check_in);

    new_room_state.set_occupancy_status(RoomDailyOccupancyStatus::Occupied);

    let mut tx = app.db.begin_tx().await;

    SqliteRoomDailyStateRepository::save(&mut tx, &new_room_state)
        .await
        .unwrap();

    tx.commit().await.unwrap();

    let response = post_json(
        &app.app,
        &format!("/reservations/{}/room-move/{}", reservation.id, new_room.id),
        &serde_json::json!({ "effective_date": reservation.check_in }),
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

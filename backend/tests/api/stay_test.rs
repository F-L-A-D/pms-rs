use axum::http::StatusCode;

use chrono::Duration;

use pms_rs::{
    api::dto::response::reservation::ReservationResponse,
    domain::entity::folio::FolioStatus,
    domain::semantic::{
        operation_change_event::OperationType,
        reservation_transition::ReservationTransitionType,
        room_daily_state::{RoomDailyHousekeepingStatus, RoomDailyOccupancyStatus, RoomDailyState},
    },
    repository::sqlite::{
        behavioral::reservation_transition_repository::SqliteReservationTransitionRepository,
        operational::{
            billing::folio_repository::SqliteFolioRepository,
            operation::operation_change_event_repository::SqliteOperationChangeEventRepository,
            room::room_daily_state_repository::SqliteRoomDailyStateRepository,
        },
    },
};

use crate::common::{
    app::{spawn_app, TestApp},
    builders::{ReservationBuilder, ReservationParticipantBuilder},
    business_date::{advance_business_date, current_open_business_date},
    client::{get, post, post_json, response_json},
    guest::create_guest,
    housekeeping::inspect_room_for_date,
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
    inspect_room_for_date(&app.app, room.id, reservation.check_in).await;

    let response = check_in(&app.app, reservation.id).await;

    assert_eq!(response.id, reservation.id,);

    assert_eq!(response.status, "checked_in",);

    let room_state = find_room_daily_state(&app, room.id, reservation.check_in).await;

    assert_eq!(
        room_state.occupancy_status,
        RoomDailyOccupancyStatus::Occupied,
    );

    let mut tx = app.db.begin_tx().await;

    let folios = SqliteFolioRepository::list_by_reservation_id(&mut tx, reservation.id)
        .await
        .unwrap();

    let _ = tx.rollback().await;

    assert_eq!(folios.len(), 1);
    assert_eq!(folios[0].status, FolioStatus::Open);

    let response = get(
        &app.app,
        &format!("/audit-logs/reservation/{}", reservation.id),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);

    let logs = response_json(response).await;

    assert!(logs
        .as_array()
        .unwrap()
        .iter()
        .any(|log| log["action"] == "stay.check_in"));
}

#[tokio::test]
async fn should_reject_check_in_when_room_is_dirty() {
    assert_check_in_rejected_by_housekeeping_status(RoomDailyHousekeepingStatus::Dirty).await;
}

#[tokio::test]
async fn should_reject_check_in_when_room_is_cleaning() {
    assert_check_in_rejected_by_housekeeping_status(RoomDailyHousekeepingStatus::Cleaning).await;
}

#[tokio::test]
async fn should_reject_check_in_when_room_is_cleaned_but_not_inspected() {
    assert_check_in_rejected_by_housekeeping_status(RoomDailyHousekeepingStatus::Cleaned).await;
}

#[tokio::test]
async fn should_allow_check_in_when_room_is_inspected() {
    let app = spawn_app().await;

    let reservation = create_reservation(&app.app).await;
    let room = create_room(&app.app).await;

    assign_room(&app.app, reservation.id, room.id).await;

    let mut room_state = RoomDailyState::new(room.id, reservation.check_in);

    room_state.inspect();

    save_room_daily_state(&app, &room_state).await;

    let response = check_in(&app.app, reservation.id).await;

    assert_eq!(response.id, reservation.id);
    assert_eq!(response.status, "checked_in");

    let room_state = find_room_daily_state(&app, room.id, reservation.check_in).await;

    assert_eq!(
        room_state.occupancy_status,
        RoomDailyOccupancyStatus::Occupied,
    );

    assert_eq!(
        room_state.housekeeping_status,
        RoomDailyHousekeepingStatus::Inspected,
    );
}

#[tokio::test]
async fn should_chek_out_reservation() {
    let app = spawn_app().await;

    let reservation = create_reservation(&app.app).await;

    let room = create_room(&app.app).await;

    assign_room(&app.app, reservation.id, room.id).await;
    inspect_room_for_date(&app.app, room.id, reservation.check_in).await;

    check_in(&app.app, reservation.id).await;
    advance_business_date(&app.app).await;

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

    let business_date = current_open_business_date(&app.app).await;
    let guest = create_guest(&app.app).await;
    let participant = ReservationParticipantBuilder::new(guest.id).build();
    let request = ReservationBuilder::new()
        .with_participant(participant)
        .with_check_in(business_date)
        .with_check_out(business_date + Duration::days(2))
        .build();

    let create_response = post_json(&app.app, "/reservations", &request).await;

    assert_eq!(create_response.status(), StatusCode::CREATED,);

    let reservation: ReservationResponse =
        serde_json::from_value(response_json(create_response).await).unwrap();

    let old_room = create_room(&app.app).await;
    let new_room = create_room(&app.app).await;

    assign_room(&app.app, reservation.id, old_room.id).await;
    inspect_room_for_date(&app.app, old_room.id, business_date).await;
    check_in(&app.app, reservation.id).await;

    let move_response = post_json(
        &app.app,
        &format!("/reservations/{}/room-move/{}", reservation.id, new_room.id),
        &serde_json::json!({ "effective_date": business_date }),
    )
    .await;

    assert_eq!(move_response.status(), StatusCode::OK,);

    let body = response_json(move_response).await;

    assert_eq!(body["status"], "room_moved");

    let old_room_state = find_room_daily_state(&app, old_room.id, business_date).await;
    let new_room_state = find_room_daily_state(&app, new_room.id, business_date).await;
    let next_new_room_state =
        find_room_daily_state(&app, new_room.id, business_date + Duration::days(1)).await;

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

    let audit_response = get(
        &app.app,
        &format!("/audit-logs/reservation/{}", reservation.id),
    )
    .await;

    assert_eq!(audit_response.status(), StatusCode::OK);

    let audit_logs = response_json(audit_response).await;

    assert!(audit_logs
        .as_array()
        .unwrap()
        .iter()
        .any(|log| log["action"] == "stay.room_move"));

    let mut tx = app.db.begin_tx().await;

    let transitions =
        SqliteReservationTransitionRepository::find_by_reservation_id(&mut tx, reservation.id)
            .await
            .unwrap();

    let operation_events = SqliteOperationChangeEventRepository::list_by_aggregate(
        &mut tx,
        "reservation",
        reservation.id,
    )
    .await
    .unwrap();

    let _ = tx.rollback().await;

    assert!(transitions.iter().any(|transition| {
        transition.transition_type == ReservationTransitionType::RoomMoved
            && transition.field_name == "room_id"
            && transition.before_value == old_room.id.to_string()
            && transition.after_value == new_room.id.to_string()
    }));

    assert!(operation_events.iter().any(|event| {
        event.operation_type == OperationType::RoomMoved
            && event.aggregate_type == "reservation"
            && event.aggregate_id == reservation.id
    }));
}

#[tokio::test]
async fn should_reject_room_move_when_target_room_is_occupied() {
    let app = spawn_app().await;

    let reservation = create_reservation(&app.app).await;
    let old_room = create_room(&app.app).await;
    let new_room = create_room(&app.app).await;

    assign_room(&app.app, reservation.id, old_room.id).await;
    inspect_room_for_date(&app.app, old_room.id, reservation.check_in).await;
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

async fn assert_check_in_rejected_by_housekeeping_status(
    housekeeping_status: RoomDailyHousekeepingStatus,
) {
    let app = spawn_app().await;

    let reservation = create_reservation(&app.app).await;
    let room = create_room(&app.app).await;

    assign_room(&app.app, reservation.id, room.id).await;

    let mut room_state = RoomDailyState::new(room.id, reservation.check_in);

    match housekeeping_status {
        RoomDailyHousekeepingStatus::Dirty => room_state.mark_dirty(),
        RoomDailyHousekeepingStatus::Cleaning => room_state.start_cleaning(),
        RoomDailyHousekeepingStatus::Cleaned => room_state.finish_cleaning(),
        RoomDailyHousekeepingStatus::Inspected => room_state.inspect(),
    }

    save_room_daily_state(&app, &room_state).await;

    let response = post(
        &app.app,
        &format!("/reservations/{}/check-in", reservation.id),
    )
    .await;

    assert_eq!(response.status(), StatusCode::CONFLICT);
}

async fn save_room_daily_state(app: &TestApp, room_state: &RoomDailyState) {
    let mut tx = app.db.begin_tx().await;

    SqliteRoomDailyStateRepository::save(&mut tx, room_state)
        .await
        .unwrap();

    tx.commit().await.unwrap();
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

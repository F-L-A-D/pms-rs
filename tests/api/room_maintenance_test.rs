use axum::http::StatusCode;

use chrono::NaiveDate;

use serde_json::json;

use pms_rs::projection::aggregate::access::{
    fetch_housekeeping_daily_workload_aggregates_by_date::fetch_housekeeping_daily_workload_aggregates_by_date,
    fetch_inventory_aggregates_by_date::fetch_inventory_aggregates_by_date,
};

use crate::common::{
    app::spawn_app,
    client::{get, post, post_json, response_json},
    reservation::create_reservation,
    room::create_room,
    stay::{assign_room, check_in},
};

#[tokio::test]
async fn should_mark_room_out_of_order_and_return_to_service() {
    let app = spawn_app().await;
    let room = create_room(&app.app).await;
    let service_date = NaiveDate::from_ymd_opt(2026, 5, 19).unwrap();
    let body = json!({ "service_date": service_date.to_string() });

    let response = post_json(&app.app, &format!("/rooms/{}/out-of-order", room.id), &body).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body_json = response_json(response).await;

    assert_eq!(body_json["room_id"], room.id.to_string());
    assert_eq!(body_json["service_date"], service_date.to_string());
    assert_eq!(body_json["occupancy_status"], "out_of_order");

    let mut tx = app.db.begin_tx().await;

    let inventory = fetch_inventory_aggregates_by_date(&mut tx, service_date)
        .await
        .unwrap();

    let housekeeping = fetch_housekeeping_daily_workload_aggregates_by_date(&mut tx, service_date)
        .await
        .unwrap();

    let _ = tx.rollback().await;

    let inventory_row = inventory
        .iter()
        .find(|aggregate| aggregate.room_class == room.room_class)
        .unwrap();

    assert_eq!(inventory_row.out_of_order_rooms, 1);
    assert_eq!(inventory_row.reservable_rooms, 0);

    let housekeeping_row = housekeeping
        .iter()
        .find(|aggregate| aggregate.room_class == room.room_class)
        .unwrap();

    assert_eq!(housekeeping_row.out_of_order_rooms, 1);

    let response = post_json(
        &app.app,
        &format!("/rooms/{}/return-to-service", room.id),
        &body,
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);

    let body_json = response_json(response).await;

    assert_eq!(body_json["occupancy_status"], "vacant");
    assert_eq!(body_json["housekeeping_status"], "dirty");

    let mut tx = app.db.begin_tx().await;

    let inventory = fetch_inventory_aggregates_by_date(&mut tx, service_date)
        .await
        .unwrap();

    let housekeeping = fetch_housekeeping_daily_workload_aggregates_by_date(&mut tx, service_date)
        .await
        .unwrap();

    let _ = tx.rollback().await;

    let inventory_row = inventory
        .iter()
        .find(|aggregate| aggregate.room_class == room.room_class)
        .unwrap();

    assert_eq!(inventory_row.out_of_order_rooms, 0);
    assert_eq!(inventory_row.reservable_rooms, 1);

    let housekeeping_row = housekeeping
        .iter()
        .find(|aggregate| aggregate.room_class == room.room_class)
        .unwrap();

    assert_eq!(housekeeping_row.out_of_order_rooms, 0);
    assert_eq!(housekeeping_row.vacant_rooms, 1);
    assert_eq!(housekeeping_row.dirty_rooms, 1);

    let response = get(
        &app.app,
        &format!("/audit-logs/room_daily_state/{}", room.id),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);

    let logs = response_json(response).await;
    let logs = logs.as_array().unwrap();

    assert!(logs.iter().any(|log| log["action"] == "room.out_of_order"));
    assert!(logs
        .iter()
        .any(|log| log["action"] == "room.return_to_service"));
}

#[tokio::test]
async fn should_reject_marking_occupied_room_out_of_order() {
    let app = spawn_app().await;

    let reservation = create_reservation(&app.app).await;
    let room = create_room(&app.app).await;

    assign_room(&app.app, reservation.id, room.id).await;
    check_in(&app.app, reservation.id).await;

    let response = post_json(
        &app.app,
        &format!("/rooms/{}/out-of-order", room.id),
        &json!({ "service_date": reservation.check_in.to_string() }),
    )
    .await;

    assert_eq!(response.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn should_reject_assigning_room_that_is_out_of_order_for_stay_date() {
    let app = spawn_app().await;

    let reservation = create_reservation(&app.app).await;
    let room = create_room(&app.app).await;

    let response = post_json(
        &app.app,
        &format!("/rooms/{}/out-of-order", room.id),
        &json!({ "service_date": reservation.check_in.to_string() }),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);

    let response = post(
        &app.app,
        &format!("/reservations/{}/assign-room/{}", reservation.id, room.id),
    )
    .await;

    assert_eq!(response.status(), StatusCode::CONFLICT);
}

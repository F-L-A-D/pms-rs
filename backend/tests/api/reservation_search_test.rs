use axum::http::StatusCode;

use pms_rs::api::dto::response::reservation::ReservationSearchItemResponse;

use crate::common::{
    app::spawn_app,
    client::{get, response_json},
    guest::create_guest,
    reservation::{
        create_reservation, create_reservation_with_external_id,
        create_reservation_with_external_id_and_guest,
    },
    room::create_room,
    stay::assign_room,
};

#[tokio::test]
async fn should_search_reservations() {
    let app = spawn_app().await;

    let reservation = create_reservation(&app.app).await;

    let response = get(&app.app, "/reservations").await;

    assert_eq!(response.status(), StatusCode::OK,);

    let items: Vec<ReservationSearchItemResponse> =
        serde_json::from_value(response_json(response).await).unwrap();

    assert!(items.iter().any(|item| item.id == reservation.id),);
}

#[tokio::test]
async fn should_filter_reservations_by_external_id() {
    let app = spawn_app().await;

    let reservation = create_reservation_with_external_id(&app.app, "booking-search-001").await;

    let response = get(&app.app, "/reservations?external_id=booking-search-001").await;

    assert_eq!(response.status(), StatusCode::OK,);

    let items: Vec<ReservationSearchItemResponse> =
        serde_json::from_value(response_json(response).await).unwrap();

    assert_eq!(items.len(), 1,);

    assert_eq!(items[0].id, reservation.id,);
}

#[tokio::test]
async fn should_filter_reservations_by_check_in_range() {
    let app = spawn_app().await;

    let reservation = create_reservation(&app.app).await;

    let response = get(
        &app.app,
        &format!(
            "/reservations?check_in_from={}&check_in_to={}",
            reservation.check_in, reservation.check_in,
        ),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK,);

    let items: Vec<ReservationSearchItemResponse> =
        serde_json::from_value(response_json(response).await).unwrap();

    assert!(items.iter().any(|item| item.id == reservation.id),);
}

#[tokio::test]
async fn should_filter_reservations_by_stay_date() {
    let app = spawn_app().await;

    let reservation = create_reservation(&app.app).await;

    let stay_date = reservation.check_in;

    let response = get(&app.app, &format!("/reservations?stay_date={}", stay_date,)).await;

    assert_eq!(response.status(), StatusCode::OK,);

    let items: Vec<ReservationSearchItemResponse> =
        serde_json::from_value(response_json(response).await).unwrap();

    assert!(items.iter().any(|item| item.id == reservation.id),);
}

#[tokio::test]
async fn should_return_linked_resources_in_reservation_search_items() {
    let app = spawn_app().await;

    let guest = create_guest(&app.app).await;

    let reservation = create_reservation_with_external_id_and_guest(
        &app.app,
        "booking-linked-resources-001",
        guest.id,
    )
    .await;

    let room = create_room(&app.app).await;

    assign_room(&app.app, reservation.id, room.id).await;

    let response = get(
        &app.app,
        "/reservations?external_id=booking-linked-resources-001",
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK,);

    let items: Vec<ReservationSearchItemResponse> =
        serde_json::from_value(response_json(response).await).unwrap();

    assert_eq!(items.len(), 1,);

    let item = &items[0];

    assert_eq!(item.id, reservation.id,);

    assert_eq!(item.linked_resources.primary_guest_id, Some(guest.id),);

    assert_eq!(item.linked_resources.assigned_room_id, Some(room.id),);
}

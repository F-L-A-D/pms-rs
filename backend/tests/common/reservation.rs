use axum::{http::StatusCode, Router};

use uuid::Uuid;

use pms_rs::{
    api::dto::response::reservation::ReservationResponse,
    domain::semantic::reservation_guest_relation::ReservationGuestRelationType,
};

use super::{
    builders::{ReservationBuilder, ReservationParticipantBuilder},
    client::{post_json, response_json},
    guest::create_guest,
};

pub async fn create_reservation(app: &Router) -> ReservationResponse {
    let guest = create_guest(app).await;

    let participant = ReservationParticipantBuilder::new(guest.id).build();

    let request = ReservationBuilder::new()
        .with_participant(participant)
        .build();

    let response = post_json(app, "/reservations", &request).await;

    assert_eq!(response.status(), StatusCode::CREATED,);

    let body = response_json(response).await;

    serde_json::from_value::<ReservationResponse>(body).unwrap()
}

pub async fn create_reservation_with_guest(app: &Router, guest_id: Uuid) -> ReservationResponse {
    let participant = ReservationParticipantBuilder::new(guest_id).build();

    let request = ReservationBuilder::new()
        .with_participant(participant)
        .build();

    let response = post_json(app, "/reservations", &request).await;

    assert_eq!(response.status(), StatusCode::CREATED,);

    let body = response_json(response).await;

    serde_json::from_value::<ReservationResponse>(body).unwrap()
}

pub async fn create_reservation_with_external_id(
    app: &Router,
    external_id: &str,
) -> ReservationResponse {
    let guest = create_guest(app).await;

    let participant = ReservationParticipantBuilder::new(guest.id).build();

    let request = ReservationBuilder::new()
        .with_external_id(external_id)
        .with_participant(participant)
        .build();

    let response = post_json(app, "/reservations", &request).await;

    assert_eq!(response.status(), StatusCode::CREATED,);

    serde_json::from_value(response_json(response).await).unwrap()
}

pub async fn create_reservation_with_external_id_and_guest(
    app: &Router,
    external_id: &str,
    guest_id: Uuid,
) -> ReservationResponse {
    let participant = ReservationParticipantBuilder::new(guest_id)
        .with_relation_type(ReservationGuestRelationType::Primary)
        .build();

    let request = ReservationBuilder::new()
        .with_external_id(external_id)
        .with_participant(participant)
        .build();

    let response = post_json(app, "/reservations", &request).await;

    assert_eq!(response.status(), StatusCode::CREATED,);

    serde_json::from_value(response_json(response).await).unwrap()
}

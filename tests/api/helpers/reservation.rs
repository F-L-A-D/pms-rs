use axum::{
    http::StatusCode,
    Router,
};

use pms_rs::api::dto::reservation::ReservationResponse;

use crate::api::helpers::{
        builders::{
            ReservationBuilder,
            ReservationParticipantBuilder,
        },

        client::{
            post_json,
            response_json,
        },

        guest::create_guest,
};

pub async fn create_reservation(
    app: &Router,
) -> ReservationResponse {

    let guest =
        create_guest(app)
            .await;

    let participant =
        ReservationParticipantBuilder::new(
            guest.id,
        )
        .build();

    let request =
        ReservationBuilder::new()
            .with_participant(participant)
            .build();

    let response =
        post_json(
            app,
            "/reservations",
            &request,
        )
        .await;

    assert_eq!(
        response.status(),
        StatusCode::CREATED,
    );

    let body =
        response_json(response)
            .await;

    serde_json::from_value::<ReservationResponse>(
        body,
    )
    .unwrap()
}
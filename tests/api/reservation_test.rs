use axum::http::StatusCode;

use chrono::{
    Duration,
    Utc,
};

use pms_rs::{
    api::dto::
        reservation::
            ReservationResponse,

    domain::
        reservation_guest_relation::
            ReservationGuestRelationType,
};

use crate::api::helpers::{
    app::spawn_app,

    builders::{
        ReservationBuilder,
        ReservationParticipantBuilder,
    },
    
    client::{
        get,
        post_json,
        response_json,
    },

    guest::create_guest,

    reservation::create_reservation,
};

#[tokio::test]
async fn should_roundtrip_reservation() {

    let app = spawn_app().await;

    let created =
        create_reservation(&app.app)
            .await;

    let response =
        get(
            &app.app,
            &format!(
                "/reservations/{}",
                created.id,
            ),
        )
        .await;

    assert_eq!(
        response.status(),
        StatusCode::OK,
    );

    let retrieved: ReservationResponse =
        serde_json::from_value(
            response_json(response)
                .await,
        )
        .unwrap();

    assert_eq!(
        retrieved.id,
        created.id,
    );

    assert_eq!(
        retrieved.room_class,
        created.room_class,
    );

    assert_eq!(
        retrieved.check_in,
        created.check_in,
    );

    assert_eq!(
        retrieved.check_out,
        created.check_out,
    );

    assert_eq!(
        retrieved.participants.len(),
        1,
    );

    assert_eq!(
        retrieved.participants[0].guest_id,
        created.participants[0].guest_id,
    );
}

#[tokio::test]
async fn should_reject_invalid_stay_range() {

    let app = spawn_app().await;

    let guest =
        create_guest(&app.app)
            .await;

    let participant =
        ReservationParticipantBuilder::new(
            guest.id,
        )
        .build();

    let today =
        Utc::now()
            .date_naive();

    let request =
        ReservationBuilder::new()
            .with_participant(participant)
            .with_check_in(
                today + Duration::days(2),
            )
            .with_check_out(
                today,
            )
            .build();

    let response =
        post_json(
            &app.app,
            "/reservations",
            &request,
        )
        .await;

    assert_eq!(
        response.status(),
        StatusCode::BAD_REQUEST,
    );
}

#[tokio::test]
async fn should_reject_empty_participants() {

    let app = spawn_app().await;

    let request =
        ReservationBuilder::new()
            .build();

    let response =
        post_json(
            &app.app,
            "/reservations",
            &request,
        )
        .await;

    assert_eq!(
        response.status(),
        StatusCode::BAD_REQUEST,
    );
}

#[tokio::test]
async fn should_reject_multiple_primary_participants() {

    let app = spawn_app().await;

    let guest1 =
        create_guest(&app.app)
            .await;

    let guest2 =
        create_guest(&app.app)
            .await;

    let participant1 =
        ReservationParticipantBuilder::new(
            guest1.id,
        )
        .with_relation_type(
            ReservationGuestRelationType::Primary,
        )
        .build();

    let participant2 =
        ReservationParticipantBuilder::new(
            guest2.id,
        )
        .with_relation_type(
            ReservationGuestRelationType::Primary,
        )
        .build();

    let request =
        ReservationBuilder::new()
            .with_participant(participant1)
            .with_participant(participant2)
            .build();

    let response =
        post_json(
            &app.app,
            "/reservations",
            &request,
        )
        .await;

    assert_eq!(
        response.status(),
        StatusCode::BAD_REQUEST,
    );
}

#[tokio::test]
async fn should_reject_duplicate_participant_guest_ids() {

    let app = spawn_app().await;

    let guest =
        create_guest(&app.app)
            .await;

    let participant1 =
        ReservationParticipantBuilder::new(
            guest.id,
        )
        .with_relation_type(
            ReservationGuestRelationType::Primary,
        )
        .build();

    let participant2 =
        ReservationParticipantBuilder::new(
            guest.id,
        )
        .with_relation_type(
            ReservationGuestRelationType::Accompany,
        )
        .build();

    let request =
        ReservationBuilder::new()
            .with_participant(participant1)
            .with_participant(participant2)
            .build();

    let response =
        post_json(
            &app.app,
            "/reservations",
            &request,
        )
        .await;

    assert_eq!(
        response.status(),
        StatusCode::BAD_REQUEST,
    );
}
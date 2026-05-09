use axum::{
    body::Body,
    http::{
        Request,
        StatusCode,
    },
};

use tower::ServiceExt;

use serde_json::Value;

use crate::helpers::{
    app::test_app,

    guest::{
        create_guest_with,
    },

    reservation::{
        create_reservation_with,
    },

    builders::{
        guest_builder::
            GuestBuilder,

        reservation_request_builder::
            ReservationRequestBuilder,

        reservation_participant_builder::
            ReservationParticipantBuilder,
    },
};

#[tokio::test]
async fn should_create_reservation_with_primary_participant() {

    let app =
        test_app().await;

    let guest_id =
        create_guest_with(
            &app.app,

            GuestBuilder::new()
                .with_name(
                    "Yamada",
                    "Taro",
                )
        )
        .await;

    let reservation_id =
        create_reservation_with(
            &app.app,

            ReservationRequestBuilder::new()
                .with_external_id(
                    "res-1",
                )
                .with_room_class(
                    "DOUBLE",
                )
                .with_primary_guest(
                    guest_id,
                )
        )
        .await;

    let response =
        app.app.clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(
                        &format!(
                            "/reservations/{}",
                            reservation_id,
                        )
                    )
                    .body(
                        Body::empty()
                    )
                    .unwrap(),
            )
            .await
            .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::OK,
    );

    let body =
        axum::body::to_bytes(
            response.into_body(),
            usize::MAX,
        )
        .await
        .unwrap();

    let reservation: Value =
        serde_json::from_slice(
            &body
        )
        .unwrap();

    assert_eq!(
        reservation["external_id"],
        "res-1",
    );

    assert_eq!(
        reservation["room_class"],
        "DOUBLE",
    );

    assert_eq!(
        reservation["participants"]
            .as_array()
            .unwrap()
            .len(),
        1,
    );
}

#[tokio::test]
async fn should_fail_when_duplicate_participants_exist() {

    let app =
        test_app().await;

    let guest_id =
        create_guest_with(
            &app.app,

            GuestBuilder::new()
                .with_name(
                    "Suzuki",
                    "Hanako",
                )
        )
        .await;

    let payload =
        ReservationRequestBuilder::new()
            .with_external_id(
                "duplicate-1",
            )
            .with_participant(
                ReservationParticipantBuilder
                    ::primary(
                        guest_id,
                    )
                    .build()
            )
            .with_participant(
                ReservationParticipantBuilder
                    ::accompany(
                        guest_id,
                    )
                    .build()
            )
            .build();

    let response =
        app.app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(
                        "/reservations"
                    )
                    .header(
                        "content-type",
                        "application/json",
                    )
                    .body(
                        Body::from(
                            payload.to_string()
                        )
                    )
                    .unwrap()
            )
            .await
            .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::BAD_REQUEST,
    );
}

#[tokio::test]
async fn should_fail_when_primary_participant_missing() {

    let app =
        test_app().await;

    let guest_id =
        create_guest_with(
            &app.app,

            GuestBuilder::new()
                .with_name(
                    "Tanaka",
                    "Jiro",
                )
        )
        .await;

    let payload =
        ReservationRequestBuilder::new()
            .with_external_id(
                "no-primary-1",
            )
            .with_participant(
                ReservationParticipantBuilder
                    ::accompany(
                        guest_id,
                    )
                    .build()
            )
            .build();

    let response =
        app.app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(
                        "/reservations"
                    )
                    .header(
                        "content-type",
                        "application/json",
                    )
                    .body(
                        Body::from(
                            payload.to_string()
                        )
                    )
                    .unwrap()
            )
            .await
            .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::BAD_REQUEST,
    );
}

#[tokio::test]
async fn should_find_reservations_by_guest_id() {

    let app =
        test_app().await;

    let target_guest_id =
        create_guest_with(
            &app.app,

            GuestBuilder::new()
                .with_name(
                    "Kobayashi",
                    "Taro",
                )
        )
        .await;

    create_guest_with(
        &app.app,

        GuestBuilder::new()
            .with_name(
                "Kobayashi",
                "Hanako",
            )
    )
    .await;

    let reservation_id =
        create_reservation_with(
            &app.app,

            ReservationRequestBuilder::new()
                .with_external_id(
                    "family-1",
                )
                .with_room_class(
                    "FAMILY",
                )
                .with_primary_guest(
                    target_guest_id,
                )
        )
        .await;

    let response =
        app.app.clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(
                        &format!(
                            "/guests/{}/reservations",
                            target_guest_id,
                        )
                    )
                    .body(
                        Body::empty()
                    )
                    .unwrap()
            )
            .await
            .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::OK,
    );

    let body =
        axum::body::to_bytes(
            response.into_body(),
            usize::MAX,
        )
        .await
        .unwrap();

    let reservations: Vec<Value> =
        serde_json::from_slice(
            &body
        )
        .unwrap();

    assert_eq!(
        reservations.len(),
        1,
    );

    assert_eq!(
        reservations[0]["id"],
        reservation_id.to_string(),
    );

    assert_eq!(
        reservations[0]["external_id"],
        "family-1",
    );
}
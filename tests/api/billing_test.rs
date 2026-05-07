use axum::{
    body::Body,
    http::{
        Request,
        StatusCode,
    },
};

use tower::ServiceExt;

use crate::helpers::{
    app::test_app,
    folio::{
        open_folio,
        post_payment,
        post_room_charge,
    },
    guest::create_guest,
    reservation::create_reservation,
};

#[tokio::test]
async fn should_open_folio() {

    let app =
        test_app().await;

    create_guest(
        &app,
        "guest-001",
    )
    .await;

    create_reservation(
        &app,
        "reservation-001",
        "guest-001",
    )
    .await;

    open_folio(
        &app,
        "folio-001",
        "reservation-001",
    )
    .await;
}

#[tokio::test]
async fn should_post_room_charge() {

    let app =
        test_app().await;

    create_guest(
        &app,
        "guest-001",
    )
    .await;

    create_reservation(
        &app,
        "reservation-001",
        "guest-001",
    )
    .await;

    open_folio(
        &app,
        "folio-001",
        "reservation-001",
    )
    .await;

    post_room_charge(
        &app,
        "folio-001",
        12000,
    )
    .await;
}

#[tokio::test]
async fn should_post_payment() {

    let app =
        test_app().await;

    create_guest(
        &app,
        "guest-001",
    )
    .await;

    create_reservation(
        &app,
        "reservation-001",
        "guest-001",
    )
    .await;

    open_folio(
        &app,
        "folio-001",
        "reservation-001",
    )
    .await;

    post_payment(
        &app,
        "folio-001",
        12000,
    )
    .await;
}

#[tokio::test]
async fn should_get_balance() {

    let app =
        test_app().await;

    create_guest(
        &app,
        "guest-001",
    )
    .await;

    create_reservation(
        &app,
        "reservation-001",
        "guest-001",
    )
    .await;

    open_folio(
        &app,
        "folio-001",
        "reservation-001",
    )
    .await;

    post_room_charge(
        &app,
        "folio-001",
        12000,
    )
    .await;

    post_payment(
        &app,
        "folio-001",
        5000,
    )
    .await;

    let response =
        app
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(
                        "/folios/folio-001/balance"
                    )
                    .body(Body::empty())
                    .unwrap()
            )
            .await
            .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::OK,
    );
}
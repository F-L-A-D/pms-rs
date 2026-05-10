use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use uuid::Uuid;

use tower::ServiceExt;

use crate::helpers::{
    app::test_app,
    billing::{
        open_folio, 
        post_payment, 
        post_room_charge,
        assign_billing_account,
        close_folio,
        create_billing_account,
        issue_invoice,
    },
    guest::create_guest,
    reservation::create_reservation,
};

use pms_rs::repository::sqlite::operational::
    folio_repository::SqliteFolioRepository;

#[tokio::test]
async fn should_open_folio() {

    let app =
        test_app().await;

    let guest_id =
        create_guest(
            &app.app
        )
        .await;

    let reservation_id =
        create_reservation(
            &app.app,
            "reservation-001",
            guest_id,
        )
        .await;

    let folio_id =
        open_folio(
            &app.app,
            reservation_id,
        )
        .await;

    assert_ne!(
        folio_id,
        uuid::Uuid::nil(),
    );
}

#[tokio::test]
async fn should_open_folio_without_billing_account() {

    let app =
        test_app().await;

    let guest_id =
        create_guest(
            &app.app
        )
        .await;

    let reservation_id =
        create_reservation(
            &app.app,
            "reservation-001",
            guest_id,
        )
        .await;

    let folio_id =
        open_folio(
            &app.app,
            reservation_id,
        )
        .await;

    let mut tx =
        app.db.begin_tx().await;

    let folio =
        SqliteFolioRepository::find_by_id(
            &mut tx,
            folio_id,
        )
        .await
        .unwrap()
        .unwrap();

    assert_eq!(
        folio.billing_account_id,
        None,
    );
}

#[tokio::test]
async fn should_persist_billing_account_assignment() {

    let app =
        test_app().await;

    let guest_id =
        create_guest(
            &app.app
        )
        .await;

    let reservation_id =
        create_reservation(
            &app.app,
            "reservation-001",
            guest_id,
        )
        .await;

    let folio_id =
        open_folio(
            &app.app,
            reservation_id,
        )
        .await;

    let billing_account_id =
        uuid::Uuid::new_v4();

    let mut tx =
        app.db.begin_tx().await;

    let mut folio =
        SqliteFolioRepository::find_by_id(
            &mut tx,
            folio_id,
        )
        .await
        .unwrap()
        .unwrap();

    folio.assign_billing_account(
        billing_account_id,
    )
    .unwrap();

    SqliteFolioRepository::save(
        &mut tx,
        &folio,
    )
    .await
    .unwrap();

    tx.commit().await.unwrap();

    let mut tx =
        app.db.begin_tx().await;

    let persisted =
        SqliteFolioRepository::find_by_id(
            &mut tx,
            folio_id,
        )
        .await
        .unwrap()
        .unwrap();

    assert_eq!(
        persisted.billing_account_id,
        Some(billing_account_id),
    );
}

#[tokio::test]
async fn should_issue_invoice_for_closed_folio() {

    let app =
        test_app().await;

    let guest_id =
        create_guest(&app.app)
            .await;

    let reservation_id =
        create_reservation(
            &app.app,
            "reservation-001",
            guest_id,
        )
        .await;

    let folio_id =
        open_folio(
            &app.app,
            reservation_id,
        )
        .await;

    post_room_charge(
        &app.app,
        folio_id,
        12000,
    )
    .await;

    let billing_account_id =
        create_billing_account(
            &app.db,
            "ACME Corp",
        )
        .await;

    assign_billing_account(
        &app.app,
        folio_id,
        billing_account_id,
    )
    .await;

    close_folio(
        &app.app,
        folio_id,
    )
    .await;

    let invoice_id =
        issue_invoice(
            &app.app,
            folio_id,
        )
        .await;

    assert_ne!(
        invoice_id,
        Uuid::nil(),
    );
}

#[tokio::test]
async fn should_post_room_charge() {
    let app = test_app().await;

    let guest_id = create_guest(&app.app).await;

    let reservation_id = create_reservation(&app.app, "reservation-001", guest_id).await;

    let folio_id =
        open_folio(
            &app.app,
            reservation_id,
        )
        .await;

    post_room_charge(&app.app, folio_id, 12000).await;
}

#[tokio::test]
async fn should_post_payment() {
    let app = test_app().await;

    let guest_id = create_guest(&app.app).await;

    let reservation_id = create_reservation(&app.app, "reservation-001", guest_id).await;

    let folio_id =
        open_folio(
            &app.app,
            reservation_id,
        )
        .await;

    post_payment(&app.app, folio_id, 12000).await;
}

#[tokio::test]
async fn should_get_balance() {
    let app = test_app().await;

    let guest_id = create_guest(&app.app).await;

    let reservation_id = create_reservation(&app.app, "reservation-001", guest_id).await;

    let folio_id =
        open_folio(
            &app.app,
            reservation_id,
        )
        .await;

    post_room_charge(&app.app, folio_id, 12000).await;

    post_payment(&app.app, folio_id, 5000).await;

    let response = app
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(
                    &format!(
                        "/folios/{}/balance",
                        folio_id
                    )
                )
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK,);
}

use axum::http::StatusCode;

use rust_decimal::Decimal;

use pms_rs::{
    api::dto::billing::{
        request::create_deposit_request::CreateDepositRequest,
        response::{folio_response::FolioResponse, payment_response::PaymentResponse},
    },
    domain::entity::{folio::FolioStatus, folio_entry::FolioEntryType, payment::PaymentMethod},
    repository::sqlite::operational::folio_entry_repository::SqliteFolioEntryRepository,
};

use crate::common::{
    app::spawn_app,
    client::{get, post, post_json, response_json},
    reservation::create_reservation,
};

#[tokio::test]
async fn should_open_reservation_folio_and_record_deposit() {
    let app = spawn_app().await;

    let reservation = create_reservation(&app.app).await;

    let response = post(
        &app.app,
        &format!("/reservations/{}/folios", reservation.id),
    )
    .await;

    assert_eq!(response.status(), StatusCode::CREATED);

    let folio: FolioResponse = serde_json::from_value(response_json(response).await).unwrap();

    assert_eq!(folio.reservation_id, reservation.id);
    assert_eq!(folio.status, FolioStatus::Open);

    let duplicate_response = post(
        &app.app,
        &format!("/reservations/{}/folios", reservation.id),
    )
    .await;

    assert_eq!(duplicate_response.status(), StatusCode::CONFLICT);

    let deposit_request = CreateDepositRequest {
        folio_id: folio.id.to_string(),
        amount: "50.00".to_string(),
        method: PaymentMethod::CreditCard,
        external_reference: Some("card-auth-001".to_string()),
    };

    let response = post_json(&app.app, "/folios/deposits", &deposit_request).await;

    assert_eq!(response.status(), StatusCode::CREATED);

    let payment: PaymentResponse = serde_json::from_value(response_json(response).await).unwrap();

    assert_eq!(payment.folio_id, folio.id);
    assert_eq!(payment.amount, Decimal::new(5000, 2));
    assert_eq!(payment.method, PaymentMethod::CreditCard);
    assert_eq!(payment.external_reference.as_deref(), Some("card-auth-001"));

    let mut tx = app.db.begin_tx().await;

    let entries = SqliteFolioEntryRepository::find_by_folio_id(&mut tx, folio.id)
        .await
        .unwrap();

    let _ = tx.rollback().await;

    assert!(entries.iter().any(|entry| {
        entry.entry_type == FolioEntryType::DepositReceived && entry.amount == Decimal::new(5000, 2)
    }));

    let folio_logs = get(&app.app, &format!("/audit-logs/folio/{}", folio.id)).await;
    assert_eq!(folio_logs.status(), StatusCode::OK);
    let folio_logs = response_json(folio_logs).await;
    assert!(folio_logs
        .as_array()
        .unwrap()
        .iter()
        .any(|log| log["action"] == "folio.open"));

    let payment_logs = get(&app.app, &format!("/audit-logs/payment/{}", payment.id)).await;
    assert_eq!(payment_logs.status(), StatusCode::OK);
    let payment_logs = response_json(payment_logs).await;
    assert!(payment_logs
        .as_array()
        .unwrap()
        .iter()
        .any(|log| log["action"] == "billing.deposit.create"));
}

#[tokio::test]
async fn should_reject_non_positive_deposit_amount() {
    let app = spawn_app().await;

    let reservation = create_reservation(&app.app).await;

    let response = post(
        &app.app,
        &format!("/reservations/{}/folios", reservation.id),
    )
    .await;

    assert_eq!(response.status(), StatusCode::CREATED);

    let folio: FolioResponse = serde_json::from_value(response_json(response).await).unwrap();

    let deposit_request = CreateDepositRequest {
        folio_id: folio.id.to_string(),
        amount: "0.00".to_string(),
        method: PaymentMethod::Cash,
        external_reference: None,
    };

    let response = post_json(&app.app, "/folios/deposits", &deposit_request).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

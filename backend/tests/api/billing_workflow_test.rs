use axum::http::StatusCode;

use rust_decimal::Decimal;

use pms_rs::{
    api::dto::billing::{
        request::create_deposit_request::CreateDepositRequest,
        response::deposit_response::DepositResponse,
    },
    domain::entity::{
        deposit::DepositStatus,
        folio::FolioStatus,
        payment::PaymentMethod,
    },
    repository::sqlite::operational::billing::folio_repository::SqliteFolioRepository,
};

use crate::common::{
    app::spawn_app,
    client::{
        get,
        post,
        post_json,
        response_json,
    },
    reservation::create_reservation,
};

#[tokio::test]
async fn should_receive_deposit_for_auto_created_reservation_folio() {
    let app =
        spawn_app().await;

    let reservation =
        create_reservation(&app.app).await;

    let mut tx =
        app.db.begin_tx().await;

    let folios =
        SqliteFolioRepository::list_by_reservation_id(
            &mut tx,
            reservation.id,
        )
        .await
        .unwrap();

    let _ =
        tx.rollback().await;

    assert_eq!(folios.len(), 1);

    let folio =
        folios
            .into_iter()
            .find(|folio| {
                folio.status == FolioStatus::Open
            })
            .unwrap();

    assert_eq!(folio.reservation_id, reservation.id);
    assert_eq!(folio.status, FolioStatus::Open);

    let duplicate_response =
        post(
            &app.app,
            &format!(
                "/reservations/{}/folios",
                reservation.id,
            ),
        )
        .await;

    assert_eq!(
        duplicate_response.status(),
        StatusCode::CONFLICT,
    );

    let deposit_request =
        CreateDepositRequest {
            folio_id: folio.id.to_string(),
            amount: "50.00".to_string(),
            method: PaymentMethod::CreditCard,
            external_reference:
                Some("card-auth-001".to_string()),
            reason: None,
        };

    let response =
        post_json(
            &app.app,
            "/folios/deposits",
            &deposit_request,
        )
        .await;

    assert_eq!(
        response.status(),
        StatusCode::CREATED,
    );

    let deposit: DepositResponse =
        serde_json::from_value(
            response_json(response).await,
        )
        .unwrap();

    assert_eq!(deposit.folio_id, folio.id);
    assert_eq!(deposit.amount, Decimal::new(5000, 2));
    assert_eq!(
        deposit.unapplied_amount,
        Decimal::new(5000, 2),
    );
    assert_eq!(
        deposit.refunded_amount,
        Decimal::ZERO,
    );
    assert_eq!(deposit.status, DepositStatus::Held);
    assert_eq!(deposit.method, PaymentMethod::CreditCard);
    assert_eq!(
        deposit.external_reference.as_deref(),
        Some("card-auth-001"),
    );

    let folio_logs =
        get(&app.app, &format!("/folios/{}/audit", folio.id)).await;

    assert_eq!(
        folio_logs.status(),
        StatusCode::OK,
    );

    let folio_logs =
        response_json(folio_logs).await;

    assert!(
        folio_logs
            .as_array()
            .unwrap()
            .iter()
            .any(|log| {
                log["action"] == "deposit.receive"
            })
    );
}

#[tokio::test]
async fn should_reject_non_positive_deposit_amount() {
    let app =
        spawn_app().await;

    let reservation =
        create_reservation(&app.app).await;

    let mut tx =
        app.db.begin_tx().await;

    let folios =
        SqliteFolioRepository::list_by_reservation_id(
            &mut tx,
            reservation.id,
        )
        .await
        .unwrap();

    let _ =
        tx.rollback().await;

    assert_eq!(folios.len(), 1);

    let folio =
        folios
            .into_iter()
            .find(|folio| {
                folio.status == FolioStatus::Open
            })
            .unwrap();

    let deposit_request =
        CreateDepositRequest {
            folio_id: folio.id.to_string(),
            amount: "0.00".to_string(),
            method: PaymentMethod::Cash,
            external_reference: None,
            reason: None,
        };

    let response =
        post_json(
            &app.app,
            "/folios/deposits",
            &deposit_request,
        )
        .await;

    assert_eq!(
        response.status(),
        StatusCode::BAD_REQUEST,
    );
}
use axum::{
    body::Body,
    http::{
        Request,
        StatusCode,
    },
    Router,
};

use serde_json::{
    json,
    Value,
};

use tower::ServiceExt;

use uuid::Uuid;

use pms_rs::{
    db::connection::Db,

    domain::billing_account::
        BillingAccount,

    repository::sqlite::operational::
        billing_account_repository::
            SqliteBillingAccountRepository,
};

pub async fn open_folio(
    app: &Router,
    reservation_id: Uuid,
) -> Uuid {

    let payload =
        json!({
            "reservation_id":
                reservation_id
        });

    let response =
        app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/folios")
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
        StatusCode::OK,
    );

    let body =
        axum::body::to_bytes(
            response.into_body(),
            usize::MAX,
        )
        .await
        .unwrap();

    let folio: Value =
        serde_json::from_slice(
            &body
        )
        .unwrap();

    Uuid::parse_str(
        folio["folio_id"]
            .as_str()
            .unwrap()
    )
    .unwrap()
}

pub async fn close_folio(
    app: &Router,
    folio_id: Uuid,
) {

    let response =
        app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(
                        &format!(
                            "/folios/{}/close",
                            folio_id
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
}

pub async fn post_room_charge(
    app: &Router,
    folio_id: Uuid,
    amount: i64,
) {

    let payload =
        json!({
            "amount": amount,
            "description": "room charge"
        });

    let response =
        app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(
                        &format!(
                            "/folios/{}/charges",
                            folio_id
                        )
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
        StatusCode::CREATED,
    );
}

pub async fn post_payment(
    app: &Router,
    folio_id: Uuid,
    amount: i64,
) {

    let payload =
        json!({
            "amount": amount,
            "description": "payment"
        });

    let response =
        app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(
                        &format!(
                            "/folios/{}/payments",
                            folio_id
                        )
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
        StatusCode::CREATED,
    );
}

pub async fn assign_billing_account(
    app: &Router,
    folio_id: Uuid,
    billing_account_id: Uuid,
) {

    let payload =
        json!({
            "billing_account_id":
                billing_account_id
        });

    let response =
        app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(
                        &format!(
                            "/folios/{}/billing-account",
                            folio_id
                        )
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
        StatusCode::OK,
    );
}

pub async fn issue_invoice(
    app: &Router,
    folio_id: Uuid,
) -> Uuid {

    let payload =
        json!({
            "folio_id":
                folio_id
        });

    let response =
        app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/invoices")
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
        StatusCode::OK,
    );

    let body =
        axum::body::to_bytes(
            response.into_body(),
            usize::MAX,
        )
        .await
        .unwrap();

    let invoice: Value =
        serde_json::from_slice(
            &body
        )
        .unwrap();

    Uuid::parse_str(
        invoice["invoice_id"]
            .as_str()
            .unwrap()
    )
    .unwrap()
}

pub async fn create_billing_account(
    db: &Db,
    name: &str,
) -> Uuid {

    let billing_account_id =
        Uuid::new_v4();

    let account =
        BillingAccount::new(
            billing_account_id,
            None,
            name.to_string(),
        )
        .unwrap();

    let mut tx =
        db.begin_tx().await;

    SqliteBillingAccountRepository::save(
        &mut tx,
        &account,
    )
    .await
    .unwrap();

    tx.commit()
        .await
        .unwrap();

    billing_account_id
}
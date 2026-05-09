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

pub async fn open_folio(
    app: &Router,
    reservation_id: Uuid,
) -> Uuid {

    let payload =
        json!({
            "reservation_id": reservation_id
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
        StatusCode::OK,
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
        StatusCode::OK,
    );
}
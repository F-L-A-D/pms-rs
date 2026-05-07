use axum::{
    body::Body,
    http::{
        Request,
        StatusCode,
    },
    Router,
};

use serde_json::json;

use tower::ServiceExt;

pub async fn open_folio(
    app: &Router,
    folio_id: &str,
    reservation_id: &str,
) {

    let payload =
        json!({
            "folio_id": folio_id,
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
}

pub async fn post_room_charge(
    app: &Router,
    folio_id: &str,
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
    folio_id: &str,
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
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

pub async fn create_reservation(
    app: &Router,
    reservation_id: &str,
    guest_id: &str,
) {

    let payload =
        json!({
            "id": reservation_id,
            "check_in": "2026-05-10",
            "nights": 2,
            "room_class": "STD",
            "primary_guest_id": guest_id
        });

    let response =
        app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/reservations")
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
use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};

use uuid::Uuid;

use serde_json::json;
use serde_json::Value;

use tower::ServiceExt;

pub async fn create_reservation(app: &Router, external_id: &str, guest_id: Uuid) -> Uuid {
    
    let payload = json!({
        "external_id": external_id,
        "check_in": "2026-05-10",
        "check_out": "2026-05-12",
        "room_class": "STD",
        "participants": [
            {
                "guest_id": guest_id,
                "relation_type": "Primary"
            }
        ]
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/reservations")
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK,);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    let json: Value = serde_json::from_slice(&body).unwrap();

    Uuid::parse_str(
        json["id"]
            .as_str()
            .unwrap()
    )
    .unwrap()
}
use axum::{
    body::Body,
    http::{
        Request,
        StatusCode,
    },
    Router,
};

use uuid::Uuid;

use serde_json::json;

use tower::ServiceExt;

#[allow(dead_code)]
pub async fn create_timeline_event(
    app: &Router,
    guest_id: Uuid,
) {

    let payload =
        json!({
            "id": "timeline-001",
            "guest_id": guest_id,
            "event_type": "ReservationCreated",
            "reference_id": "reservation-001"
        });

    let response =
        app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/internal/timeline-events")
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
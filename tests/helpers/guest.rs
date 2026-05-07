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

pub async fn create_guest(
    app: &Router,
    guest_id: &str,
) {

    let payload =
        json!({
            "id": guest_id,
            "last_name": "Yamada",
            "first_name": "Taro"
        });

    let response =
        app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/guests")
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
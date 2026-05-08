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

use uuid::Uuid;

pub async fn create_guest(
    app: &Router,
) -> Uuid {

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
                            json!({
                                "last_name": "Yamada",
                                "first_name": "Taro"
                            })
                            .to_string()
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

    let json: serde_json::Value =
        serde_json::from_slice(&body)
            .unwrap();

    Uuid::parse_str(
        json["id"]
            .as_str()
            .unwrap()
    )
    .unwrap()
}
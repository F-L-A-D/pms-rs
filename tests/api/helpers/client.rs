use axum::{
    body::{
        Body,
        to_bytes,
    },
    http::{
        Request,
        Response,
    },
};

use serde::Serialize;

use serde_json::Value;

use tower::ServiceExt;

#[allow(dead_code)]
pub async fn get(
    app: &axum::Router,
    uri: &str,
) -> Response<Body> {

    app.clone()
        .oneshot(
            Request::builder()
                .uri(uri)
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap()
}

#[allow(dead_code)]
pub async fn post(
    app: &axum::Router,
    uri: &str,
) -> Response<Body> {

    app.clone()
        .oneshot(
            Request::builder()
                .uri(uri)
                .method("POST")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap()
}

#[allow(dead_code)]
pub async fn post_json<T>(
    app: &axum::Router,
    uri: &str,
    body: &T,
) -> Response<Body>
where
    T: Serialize,
{
    let json =
        serde_json::to_vec(body)
            .unwrap();

    app.clone()
        .oneshot(
            Request::builder()
                .uri(uri)
                .method("POST")
                .header(
                    "content-type",
                    "application/json",
                )
                .body(Body::from(json))
                .unwrap(),
        )
        .await
        .unwrap()
}

#[allow(dead_code)]
pub async fn response_json(
    response: Response<Body>,
) -> Value {

    let bytes =
        to_bytes(
            response.into_body(),
            usize::MAX,
        )
        .await
        .unwrap();

    serde_json::from_slice(&bytes)
        .unwrap()
}

#[allow(dead_code)]
pub async fn put_json<T>(
    app: &axum::Router,
    uri: &str,
    body: &T,
) -> Response<Body>
where
    T: Serialize,
{
    let json =
        serde_json::to_vec(body)
            .unwrap();

    app.clone()
        .oneshot(
            Request::builder()
                .uri(uri)
                .method("PUT")
                .header(
                    "content-type",
                    "application/json",
                )
                .body(Body::from(json))
                .unwrap(),
        )
        .await
        .unwrap()
}

pub async fn patch_json<T>(
    app: &axum::Router,
    uri: &str,
    body: &T,
) -> Response<Body>
where
    T: Serialize,
{
    let json =
        serde_json::to_vec(body)
            .unwrap();

    app.clone()
        .oneshot(
            Request::builder()
                .uri(uri)
                .method("PATCH")
                .header(
                    "content-type",
                    "application/json",
                )
                .body(Body::from(json))
                .unwrap(),
        )
        .await
        .unwrap()
}
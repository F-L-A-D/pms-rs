use axum::{
    extract::{
        Path,
        State,
    },

    http::StatusCode,

    Json,
};

use uuid::Uuid;

use crate::{
    api::{
        dto::billing::{
            PostPaymentRequest,
            PostRoomChargeRequest,
        },

        error::{
            ApiError,
            map_app_error,
        },

        state::AppState,
    },

    usecase::billing::command::{
        post_payment::
            post_payment,

        post_room_charge::
            post_room_charge,
    },
};

pub async fn post_payment_handler(
    State(state): State<AppState>,
    Path(folio_id): Path<String>,
    Json(request): Json<PostPaymentRequest>,
) -> Result<
    StatusCode,
    ApiError,
> {

    let folio_id =
        Uuid::parse_str(&folio_id)
            .map_err(|e| {
                ApiError::new(
                    StatusCode::BAD_REQUEST,
                    e.to_string(),
                )
            })?;

    post_payment(
        &state.db,
        folio_id,
        request.amount,
        request.description,
    )
    .await
    .map_err(map_app_error)?;

    Ok(StatusCode::CREATED)
}

pub async fn post_room_charge_handler(
    State(state): State<AppState>,
    Path(folio_id): Path<String>,
    Json(request): Json<PostRoomChargeRequest>,
) -> Result<
    StatusCode,
    ApiError,
> {

    let folio_id =
        Uuid::parse_str(&folio_id)
            .map_err(|e| {
                ApiError::new(
                    StatusCode::BAD_REQUEST,
                    e.to_string(),
                )
            })?;

    post_room_charge(
        &state.db,
        folio_id,
        request.amount,

        Some(request.description),
    )
    .await
    .map_err(map_app_error)?;

    Ok(StatusCode::CREATED)
}
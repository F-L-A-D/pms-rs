use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};

use chrono::NaiveDate;

use uuid::Uuid;

use crate::{
    api::{
        dto::{
            input::guest::{CreateGuestInput, GuestSearchInput, UpdateGuestInput},
            request::guest::{CreateGuestRequest, GuestSearchQuery, UpdateGuestRequest},
            response::guest::GuestResponse,
        },
        error::{map_app_error, ApiError},
        state::AppState,
    },
    usecase::guest::{
        command::{create_guest::create_guest, update_guest::update_guest},
        detail::get_guest::get_guest,
        search::get_guests::get_guests,
    },
};

pub async fn create_guest_handler(
    State(state): State<AppState>,
    Json(req): Json<CreateGuestRequest>,
) -> Result<(StatusCode, Json<GuestResponse>), ApiError> {
    let birth_date = req
        .birth_date
        .map(|v| NaiveDate::parse_from_str(&v, "%Y-%m-%d"))
        .transpose()
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    let input = CreateGuestInput {
        last_name: req.last_name,

        first_name: req.first_name,

        phone: req.phone,

        email: req.email,

        nationality: req.nationality,

        birth_date: birth_date,

        gender: req.gender,

        membership_code: req.membership_code,

        marketing_opt_in: req.marketing_opt_in,
    };

    let guest = create_guest(&state.db, input)
        .await
        .map_err(map_app_error)?;

    let response = GuestResponse::from(guest);

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_guest_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<GuestResponse>, ApiError> {
    let guest_id =
        Uuid::parse_str(&id).map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    let guest = get_guest(&state.db, guest_id)
        .await
        .map_err(map_app_error)?
        .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "guest not found"))?;

    Ok(Json(guest.into()))
}

pub async fn get_guests_handler(
    State(state): State<AppState>,
    Query(query): Query<GuestSearchQuery>,
) -> Result<Json<Vec<GuestResponse>>, ApiError> {
    let input = GuestSearchInput {
        query: query.query,

        field: query.field,
    };

    let guests = get_guests(&state.db, input).await.map_err(map_app_error)?;

    let response = guests.into_iter().map(Into::into).collect();

    Ok(Json(response))
}

pub async fn update_guest_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateGuestRequest>,
) -> Result<Json<GuestResponse>, ApiError> {
    let guest_id =
        Uuid::parse_str(&id).map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    let birth_date = req
        .birth_date
        .map(|v| NaiveDate::parse_from_str(&v, "%Y-%m-%d"))
        .transpose()
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    let input = UpdateGuestInput {
        guest_id,

        last_name: req.last_name,

        first_name: req.first_name,

        phone: req.phone,

        email: req.email,

        nationality: req.nationality,

        birth_date: birth_date,

        gender: req.gender,

        membership_code: req.membership_code,

        marketing_opt_in: req.marketing_opt_in,
    };

    let guest = update_guest(&state.db, input)
        .await
        .map_err(map_app_error)?;

    Ok(Json(guest.into()))
}

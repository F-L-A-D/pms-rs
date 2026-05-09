use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;

use crate::api::dto::guest::{
    CreateGuestRequest, GuestResponse, GuestSearchQuery, UpdateGuestRequest,
};

use crate::api::error::{map_app_error, ApiError};
use crate::api::state::AppState;

use crate::domain::guest::Guest;

use crate::error::app_error::AppError;

use crate::usecase::guest::{
    create_guest::create_guest, get_guest::get_guest, list_guests::list_guests,
    update_guest::update_guest,
};

pub async fn create_guest_handler(
    State(state): State<AppState>,
    Json(req): Json<CreateGuestRequest>,
) -> Result<Json<GuestResponse>, ApiError> {
    let guest = Guest::new(
        Uuid::new_v4(),
        req.last_name,
        req.first_name,
        req.phone,
        req.email,
        req.nationality,
        req.birth_date,
        req.gender,
        req.membership_code,
        req.marketing_opt_in,
    )
    .map_err(|e| map_app_error(AppError::Validation(e)))?;

    create_guest(&state.db, guest.clone())
        .await
        .map_err(map_app_error)?;

    Ok(Json(guest.into()))
}

pub async fn get_guest_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<GuestResponse>, ApiError> {
    let guest_id =
        Uuid::parse_str(&id).map_err(|e| map_app_error(AppError::Validation(e.to_string())))?;

    let guest = get_guest(&state.db, guest_id)
        .await
        .map_err(map_app_error)?
        .ok_or_else(|| ApiError::new(axum::http::StatusCode::NOT_FOUND, "guest not found"))?;

    Ok(Json(guest.into()))
}

pub async fn list_guests_handler(
    State(state): State<AppState>,
    Query(query): Query<GuestSearchQuery>,
) -> Result<Json<Vec<GuestResponse>>, ApiError> {

    let guests =
        list_guests(
            &state.db,
            query.query,
            query.field,
        )
        .await
        .map_err(map_app_error)?;

    let response =
        guests
            .into_iter()
            .map(Into::into)
            .collect();

    Ok(Json(response))
}

pub async fn update_guest_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateGuestRequest>,
) -> Result<Json<GuestResponse>, ApiError> {
    let guest_id =
        Uuid::parse_str(&id).map_err(|e| map_app_error(AppError::Validation(e.to_string())))?;

    let guest = update_guest(
        &state.db,
        guest_id,
        req.last_name,
        req.first_name,
        req.phone,
        req.email,
        req.nationality,
        req.birth_date,
        req.gender,
        req.membership_code,
        req.marketing_opt_in,
    )
    .await
    .map_err(map_app_error)?;

    Ok(Json(guest.into()))
}

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};

use uuid::Uuid;

use crate::{
    api::{
        dto::guest::{
            CreateGuestRequest,
            GuestResponse,
            GuestSearchQuery,
            UpdateGuestRequest,
        },

        error::{
            map_app_error,
            ApiError,
        },

        state::AppState,
    },
    domain::guest::{
        Guest,
        GuestProfileUpdate,
    },
    usecase::guest::{
        command::{
            create_guest::create_guest,
            update_guest::update_guest,
        },

        detail::get_guest::get_guest,

        search::get_guests::get_guests,
    },
};

pub async fn create_guest_handler(
    State(state): State<AppState>,
    Json(req): Json<CreateGuestRequest>,
) -> Result<
    (StatusCode, Json<GuestResponse>),
    ApiError,
> {

    let profile =
        GuestProfileUpdate {
            last_name: req.last_name,

            first_name: req.first_name,

            phone: req.phone,

            email: req.email,

            nationality: req.nationality,

            birth_date: req.birth_date,

            gender: req.gender,

            membership_code: req.membership_code,

            marketing_opt_in:
                req.marketing_opt_in,
        };

    let guest =
        Guest::new(
            Uuid::new_v4(),
            profile,
        )
        .map_err(|e| {
            ApiError::new(
                axum::http::StatusCode::BAD_REQUEST,
                e,
            )
        })?;

    let response =
        GuestResponse::from(
            guest.clone(),
        );

    create_guest(
        &state.db,
        guest,
    )
    .await
    .map_err(map_app_error)?;

    Ok((
        StatusCode::CREATED,
        Json(response),
    ))
}

pub async fn get_guest_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<GuestResponse>, ApiError> {

    let guest_id =
        Uuid::parse_str(&id)
            .map_err(|e| {
                ApiError::new(
                    axum::http::StatusCode::BAD_REQUEST,
                    e.to_string(),
                )
            })?;

    let guest =
        get_guest(
            &state.db,
            guest_id,
        )
        .await
        .map_err(map_app_error)?
        .ok_or_else(|| {
            ApiError::new(
                axum::http::StatusCode::NOT_FOUND,
                "guest not found",
            )
        })?;

    Ok(
        Json(
            guest.into(),
        )
    )
}

pub async fn get_guests_handler(
    State(state): State<AppState>,
    Query(query): Query<GuestSearchQuery>,
) -> Result<Json<Vec<GuestResponse>>, ApiError> {

    let guests =
        get_guests(
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
        Uuid::parse_str(&id)
            .map_err(|e| {
                ApiError::new(
                    axum::http::StatusCode::BAD_REQUEST,
                    e.to_string(),
                )
            })?;

    let profile =
        GuestProfileUpdate {
            last_name: req.last_name,

            first_name: req.first_name,

            phone: req.phone,

            email: req.email,

            nationality: req.nationality,

            birth_date: req.birth_date,

            gender: req.gender,

            membership_code: req.membership_code,

            marketing_opt_in:
                req.marketing_opt_in,
        };

    let guest =
        update_guest(
            &state.db,
            guest_id,
            profile,
        )
        .await
        .map_err(map_app_error)?;

    Ok(
        Json(
            guest.into(),
        )
    )
}
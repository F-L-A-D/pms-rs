use axum::{
    extract::{
        Path,
        State,
        Query,
    },
    http::StatusCode,
    Json,
};

use crate::api::dto::guest::{
    CreateGuestRequest,
    GuestResponse,
    UpdateGuestRequest,
    GuestSearchQuery,
};

use crate::api::state::AppState;

use crate::domain::guest::Guest;

use crate::repository::sqlite::guest_repository::SqliteGuestRepository;

pub async fn create_guest_handler(
    State(state): State<AppState>,
    Json(req): Json<CreateGuestRequest>,
) -> Result<Json<GuestResponse>, StatusCode> {

    let guest =
        Guest::new(
            req.id,
            req.last_name,
            req.first_name,
            req.phone,
            req.email,
        )
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    SqliteGuestRepository::save(
        &state.db.pool,
        &guest,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(
        Json(
            GuestResponse {
                id: guest.id,
                last_name: guest.last_name,
                first_name: guest.first_name,
                phone: guest.phone,
                email: guest.email,
            }
        )
    )
}

pub async fn get_guest_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<GuestResponse>, StatusCode> {

    let guest =
        SqliteGuestRepository::find_by_id(
            &state.db.pool,
            &id,
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(
        Json(
            GuestResponse {
                id: guest.id,
                last_name: guest.last_name,
                first_name: guest.first_name,
                phone: guest.phone,
                email: guest.email,
            }
        )
    )
}

pub async fn list_guests_handler(
    State(state): State<AppState>,
    Query(query): Query<GuestSearchQuery>,
) -> Result<Json<Vec<GuestResponse>>, StatusCode> {

    let guests =

        if let Some(name) = query.name {

            SqliteGuestRepository::find_by_name(
                &state.db.pool,
                &name,
            )
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?

        } else {

            SqliteGuestRepository::find_all(
                &state.db.pool,
            )
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?

        };

    let response =
        guests
            .into_iter()
            .map(|guest| {

                GuestResponse {
                    id: guest.id,
                    last_name: guest.last_name,
                    first_name: guest.first_name,
                    phone: guest.phone,
                    email: guest.email,
                }

            })
            .collect();

    Ok(Json(response))
}

pub async fn update_guest_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateGuestRequest>,
) -> Result<Json<GuestResponse>, StatusCode> {

    let mut guest =
        SqliteGuestRepository::find_by_id(
            &state.db.pool,
            &id,
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    guest.update_profile(
        req.last_name,
        req.first_name,
        req.phone,
        req.email,
    )
    .map_err(|_| StatusCode::BAD_REQUEST)?;

    SqliteGuestRepository::update(
        &state.db.pool,
        &guest,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(
        Json(
            GuestResponse {
                id: guest.id,
                last_name: guest.last_name,
                first_name: guest.first_name,
                phone: guest.phone,
                email: guest.email,
            }
        )
    )
}
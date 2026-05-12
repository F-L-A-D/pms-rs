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
            BalanceResponse,
            FolioEntryResponse,
            FolioResponse,
            OpenFolioRequest,
        },

        error::{
            map_app_error,
            ApiError,
        },

        state::AppState,
    },

    usecase::billing::{
        calculation::calculate_balance::
            calculate_balance,

        command::{
            close_folio::close_folio,
            open_folio::open_folio,
        },

        search::get_folio_entries::
            get_folio_entries,
    },
};

pub async fn open_folio_handler(
    State(state): State<AppState>,
    Json(request): Json<OpenFolioRequest>,
) -> Result<
    Json<FolioResponse>,
    ApiError,
> {

    let folio =
        open_folio(
            &state.db,
            request.reservation_id,
        )
        .await
        .map_err(map_app_error)?;

    Ok(
        Json(
            folio.into()
        )
    )
}

pub async fn close_folio_handler(
    State(state): State<AppState>,
    Path(folio_id): Path<String>,
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

    close_folio(
        &state.db,
        folio_id,
    )
    .await
    .map_err(map_app_error)?;

    Ok(StatusCode::OK)
}

pub async fn get_balance_handler(
    State(state): State<AppState>,
    Path(folio_id): Path<String>,
) -> Result<
    Json<BalanceResponse>,
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

    let balance =
        calculate_balance(
            &state.db,
            folio_id,
        )
        .await
        .map_err(map_app_error)?;

    Ok(
        Json(
            BalanceResponse {
                folio_id,
                balance,
            }
        )
    )
}

pub async fn get_entries_handler(
    State(state): State<AppState>,
    Path(folio_id): Path<String>,
) -> Result<
    Json<Vec<FolioEntryResponse>>,
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

    let entries =
        get_folio_entries(
            &state.db,
            folio_id,
        )
        .await
        .map_err(map_app_error)?;

    let response =
        entries
            .into_iter()
            .map(Into::into)
            .collect();

    Ok(Json(response))
}
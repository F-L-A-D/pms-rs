use axum::{
    extract::{
        Path,
        State,
    },

    http::StatusCode,

    response::{
        IntoResponse,
        Response,
    },

    Json,
};

use uuid::Uuid;

use crate::{
    api::{
        dto::billing::{
            BalanceResponse,
            FolioResponse,
            OpenFolioRequest,
            FolioEntryResponse,
        },

        error::{
            ApiError,
            map_app_error,
        },
    },

    api::state::AppState,

    error::app_error::AppError,

    usecase::billing::{
        close_folio::close_folio,
        calculate_balance::calculate_balance,
        open_folio::open_folio,
        get_folio_entries::
            get_folio_entries,
    }
};

pub async fn open_folio_handler(

    State(state): State<AppState>,

    Json(request): Json<OpenFolioRequest>,

) -> Result<Response, ApiError> {

    let folio =
        open_folio(
            &state.db,
            request.reservation_id,
        )
        .await
        .map_err(map_app_error)?;

    Ok(
        Json(
            FolioResponse {
                folio_id:
                    folio.id,

                status:
                    format!("{:?}", folio.status),
            }
        )
        .into_response()
    )
}

pub async fn close_folio_handler(

    State(state): State<AppState>,

    Path(folio_id): Path<String>,

) -> Result<Response, ApiError> {

    let folio_id =
        Uuid::parse_str(&folio_id)
            .map_err(|e| {
                map_app_error(
                    AppError::Validation(
                        e.to_string()
                    )
                )
            })?;

    close_folio(
        &state.db,
        folio_id,
    )
    .await
    .map_err(map_app_error)?;

    Ok(
        StatusCode::OK
            .into_response()
    )
}

pub async fn get_balance_handler(

    State(state): State<AppState>,

    Path(folio_id): Path<String>,

) -> Result<Response, ApiError> {

    let folio_id =
        Uuid::parse_str(&folio_id)
            .map_err(|e| {
                map_app_error(
                    AppError::Validation(
                        e.to_string()
                    )
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
        .into_response()
    )
}

pub async fn get_entries_handler(

    State(state): State<AppState>,

    Path(folio_id): Path<String>,

) -> Result<Response, ApiError> {

    let folio_id =
        Uuid::parse_str(&folio_id)
            .map_err(|e| {
                map_app_error(
                    AppError::Validation(
                        e.to_string()
                    )
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
            .map(|entry| {
                FolioEntryResponse {

                    id:
                        entry.id,

                    entry_type:
                        format!(
                            "{:?}",
                            entry.entry_type
                        ),

                    amount:
                        entry.amount,

                    description:
                        entry.description,

                    occurred_at:
                        entry.occurred_at,
                }
            })
            .collect::<Vec<_>>();

    Ok(
        Json(response)
            .into_response()
    )
}
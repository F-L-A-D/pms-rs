use axum::{
    extract::{Path, State},
    Json,
};

use uuid::Uuid;

use crate::api::dto::billing::{
    BalanceResponse, FolioEntryResponse, FolioResponse, OpenFolioRequest, PostPaymentRequest,
    PostRoomChargeRequest,
};

use crate::api::error::{map_app_error, ApiError};

use crate::error::app_error::AppError;

use crate::api::state::AppState;

use crate::usecase::billing::{
    calculate_balance::calculate_balance, open_folio::open_folio,
    post_room_charge::post_room_charge,
};

pub async fn open_folio_handler(
    State(state): State<AppState>,
    Json(req): Json<OpenFolioRequest>,
) -> Result<Json<FolioResponse>, ApiError> {

    let folio = 
        open_folio(&state.db, req.reservation_id)
            .await
            .map_err(map_app_error)?;

    Ok(Json(FolioResponse {
        folio_id: folio.id,
        status: "opened".into(),
    }))
}

pub async fn post_room_charge_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<PostRoomChargeRequest>,
) -> Result<Json<FolioResponse>, ApiError> {

    let folio_id =
        Uuid::parse_str(&id).map_err(|e| map_app_error(AppError::Validation(e.to_string())))?;

    post_room_charge(
        &state.db,
        folio_id,
        req.amount,
        Some(req.description),
    )
    .await
    .map_err(map_app_error)?;

    Ok(Json(FolioResponse {
        folio_id,
        status: "charge_posted".into(),
    }))
}

pub async fn post_payment_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<PostPaymentRequest>,
) -> Result<Json<FolioResponse>, ApiError> {

    let folio_id =
        Uuid::parse_str(&id).map_err(|e| map_app_error(AppError::Validation(e.to_string())))?;

    post_room_charge(
        &state.db,
        folio_id,
        -req.amount,
        Some(req.description),
    )
    .await
    .map_err(map_app_error)?;

    Ok(Json(FolioResponse {
        folio_id,
        status: "payment_posted".into(),
    }))
}

pub async fn get_balance_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<BalanceResponse>, ApiError> {

    let folio_id =
        Uuid::parse_str(&id).map_err(|e| map_app_error(AppError::Validation(e.to_string())))?;

    let balance = calculate_balance(&state.db, folio_id)
        .await
        .map_err(map_app_error)?;

    Ok(Json(BalanceResponse { folio_id, balance }))
}

pub async fn get_entries_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Vec<FolioEntryResponse>>, ApiError> {

    let folio_id =
        Uuid::parse_str(&id).map_err(|e| map_app_error(AppError::Validation(e.to_string())))?;

    let mut tx = state.db.begin_tx().await;

    let entries =
        crate::repository::sqlite::operational::
            folio_entry_repository::SqliteFolioEntryRepository::find_by_folio_id(
                &mut tx,
                folio_id,
            )
            .await
            .map_err(|_| {
                ApiError::new(
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "internal server error",
                )
            })?;

    tx.rollback().await.map_err(|_| {
        ApiError::new(
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "internal server error",
        )
    })?;

    let response = entries
        .into_iter()
        .map(|entry| FolioEntryResponse {
            id: entry.id,

            entry_type: format!("{:?}", entry.entry_type),

            amount: entry.amount,

            description: entry.description,

            occurred_at: entry.occurred_at,
        })
        .collect();

    Ok(Json(response))
}

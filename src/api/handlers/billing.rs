use axum::{
    extract::{
        Path,
        State,
    },
    http::StatusCode,
    Json,
};

use chrono::Utc;

use crate::api::dto::billing::{
    BalanceResponse,
    FolioEntryResponse,
    FolioResponse,
    OpenFolioRequest,
    PostPaymentRequest,
    PostRoomChargeRequest,
};

use crate::api::error::map_app_error;

use crate::api::state::AppState;

use crate::usecase::billing::{
    calculate_balance::calculate_balance,
    open_folio::open_folio,
    post_room_charge::post_room_charge,
};

pub async fn open_folio_handler(
    State(state): State<AppState>,
    Json(req): Json<OpenFolioRequest>,
) -> Result<Json<FolioResponse>, StatusCode> {

    open_folio(
        &state.db,
        req.folio_id.clone(),
        req.reservation_id.clone(),
    )
    .await
    .map_err(map_app_error)?;

    Ok(
        Json(
            FolioResponse {
                folio_id: req.folio_id,
                status: "opened".into(),
            }
        )
    )
}

pub async fn post_room_charge_handler(
    State(state): State<AppState>,
    Path(folio_id): Path<String>,
    Json(req): Json<PostRoomChargeRequest>,
) -> Result<Json<FolioResponse>, StatusCode> {

    post_room_charge(
        &state.db,
        format!(
            "charge-{}",
            Utc::now().timestamp_millis()
        ),
        &folio_id,
        req.amount,
        Some(req.description),
    )
    .await
    .map_err(map_app_error)?;

    Ok(
        Json(
            FolioResponse {
                folio_id,
                status: "charge_posted".into(),
            }
        )
    )
}

pub async fn post_payment_handler(
    State(state): State<AppState>,
    Path(folio_id): Path<String>,
    Json(req): Json<PostPaymentRequest>,
) -> Result<Json<FolioResponse>, StatusCode> {

    post_room_charge(
        &state.db,
        format!(
            "payment-{}",
            Utc::now().timestamp_millis()
        ),
        &folio_id,
        -req.amount,
        Some(req.description),
    )
    .await
    .map_err(map_app_error)?;

    Ok(
        Json(
            FolioResponse {
                folio_id,
                status: "payment_posted".into(),
            }
        )
    )
}

pub async fn get_balance_handler(
    State(state): State<AppState>,
    Path(folio_id): Path<String>,
) -> Result<Json<BalanceResponse>, StatusCode> {

    let balance =
        calculate_balance(
            &state.db,
            &folio_id,
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
) -> Result<Json<Vec<FolioEntryResponse>>, StatusCode> {

    let mut tx =
        state.db.begin_tx().await;

    let entries =
        crate::repository::sqlite::folio_entry_repository::SqliteFolioEntryRepository::find_by_folio_id(
            &mut tx,
            &folio_id,
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tx.rollback()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response =
        entries
            .into_iter()
            .map(|entry| {

                FolioEntryResponse {
                    id: entry.id,

                    entry_type:
                        format!(
                            "{:?}",
                            entry.entry_type
                        ),

                    amount: entry.amount,

                    description:
                        entry.description,

                    occurred_at:
                        entry.occurred_at,
                }

            })
            .collect();

    Ok(Json(response))
}
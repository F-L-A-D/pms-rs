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
    OpenFolioRequest,
    PostRoomChargeRequest,
    PostPaymentRequest,
    FolioResponse,
    BalanceResponse,
    FolioEntryResponse,
};

use crate::api::state::AppState;

use crate::domain::folio::Folio;
use crate::domain::folio_entry::{FolioEntry, EntryType};


use crate::repository::sqlite::folio_repository::SqliteFolioRepository;
use crate::repository::sqlite::folio_entry_repository::SqliteFolioEntryRepository;

use crate::usecase::billing::{
    calculate_balance::calculate_balance,
};

pub async fn open_folio(
    State(state): State<AppState>,
    Json(req): Json<OpenFolioRequest>,
) -> Result<Json<FolioResponse>, StatusCode> {

    let folio =
        Folio::new(
            req.folio_id.clone(),
            req.reservation_id.clone(),
        );

    SqliteFolioRepository::save(
        &state.db.pool,
        &folio,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(
        Json(
            FolioResponse {
                folio_id: req.folio_id,
                status: "opened".into(),
            }
        )
    )
}

pub async fn post_room_charge(
    State(state): State<AppState>,
    Path(folio_id): Path<String>,
    Json(req): Json<PostRoomChargeRequest>,
) -> Result<Json<FolioResponse>, StatusCode> {

    let entry =
        FolioEntry::new(
            format!("charge-{}", Utc::now().timestamp_millis()),
            folio_id.clone(),
            EntryType::RoomCharge,
            req.amount,
            Some(req.description),
        );

    SqliteFolioEntryRepository::save(
        &state.db.pool,
        &entry,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(
        Json(
            FolioResponse {
                folio_id,
                status: "charge_posted".into(),
            }
        )
    )
}

pub async fn post_payment(
    State(state): State<AppState>,
    Path(folio_id): Path<String>,
    Json(req): Json<PostPaymentRequest>,
) -> Result<Json<FolioResponse>, StatusCode> {

    let entry =
        FolioEntry::new(
            format!("payment-{}", Utc::now().timestamp_millis()),
            folio_id.clone(),
            EntryType::Payment,
            req.amount,
            Some(req.description),
        );

    SqliteFolioEntryRepository::save(
        &state.db.pool,
        &entry,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(
        Json(
            FolioResponse {
                folio_id,
                status: "payment_posted".into(),
            }
        )
    )
}

pub async fn get_balance(
    State(state): State<AppState>,
    Path(folio_id): Path<String>,
) -> Result<Json<BalanceResponse>, StatusCode> {

    let balance =
        calculate_balance(
            &state.db,
            &folio_id,
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(
        Json(
            BalanceResponse {
                folio_id,
                balance,
            }
        )
    )
}

pub async fn get_entries(
    State(state): State<AppState>,
    Path(folio_id): Path<String>,
) -> Result<Json<Vec<FolioEntryResponse>>, StatusCode> {

    let entries =
        SqliteFolioEntryRepository::find_by_folio_id(
            &state.db.pool,
            &folio_id,
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response =
        entries
            .into_iter()
            .map(|entry| {

                FolioEntryResponse {
                    id: entry.id,
                    entry_type:
                        format!("{:?}", entry.entry_type),

                    amount: entry.amount,
                    description: entry.description,
                    occurred_at: entry.occurred_at,
                }

            })
            .collect();

    Ok(Json(response))
}
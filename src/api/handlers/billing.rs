use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use uuid::Uuid;

use crate::{
    api::{
        dto::billing::{
            AssignBillingAccountRequest,
            BalanceResponse,
            FolioEntryResponse,
            FolioResponse,
            IssueInvoiceResponse,
            OpenFolioRequest,
            PostPaymentRequest,
            PostRoomChargeRequest,
        },
        error::{map_app_error, ApiError},
        state::AppState,
    },

    error::app_error::AppError,

    repository::sqlite::operational::{
        folio_entry_repository::
            SqliteFolioEntryRepository,

        folio_repository::
            SqliteFolioRepository,
    },

    usecase::billing::{
        calculate_balance::calculate_balance,

        issue_invoice::{
            issue_invoice,
            IssueInvoiceInput,
        },

        open_folio::open_folio,
        close_folio::close_folio,
        post_room_charge::post_room_charge,
    },
};

pub async fn open_folio_handler(
    State(state): State<AppState>,
    Json(req): Json<OpenFolioRequest>,
) -> Result<Json<FolioResponse>, ApiError> {

    let folio =
        open_folio(
            &state.db,
            req.reservation_id,
        )
        .await
        .map_err(map_app_error)?;

    Ok(Json(FolioResponse {

        folio_id:
            folio.id,

        status:
            "opened".into(),
    }))
}

pub async fn close_folio_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {

    let folio_id =
        Uuid::parse_str(&id)
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
    .map_err(|e| {
        map_app_error(
            AppError::Validation(e)
        )
    })?;

    Ok(StatusCode::OK)
}

pub async fn post_room_charge_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<PostRoomChargeRequest>,
) -> Result<Json<FolioResponse>, ApiError> {

    let folio_id =
        Uuid::parse_str(&id)
            .map_err(|e| {
                map_app_error(
                    AppError::Validation(
                        e.to_string()
                    )
                )
            })?;

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

        status:
            "charge_posted".into(),
    }))
}

pub async fn post_payment_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<PostPaymentRequest>,
) -> Result<Json<FolioResponse>, ApiError> {

    let folio_id =
        Uuid::parse_str(&id)
            .map_err(|e| {
                map_app_error(
                    AppError::Validation(
                        e.to_string()
                    )
                )
            })?;

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

        status:
            "payment_posted".into(),
    }))
}

pub async fn get_balance_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<BalanceResponse>, ApiError> {

    let folio_id =
        Uuid::parse_str(&id)
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

    Ok(Json(BalanceResponse {

        folio_id,

        balance,
    }))
}

pub async fn get_entries_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Vec<FolioEntryResponse>>, ApiError> {

    let folio_id =
        Uuid::parse_str(&id)
            .map_err(|e| {
                map_app_error(
                    AppError::Validation(
                        e.to_string()
                    )
                )
            })?;

    let mut tx =
        state.db.begin_tx().await;

    let entries =
        SqliteFolioEntryRepository::find_by_folio_id(
            &mut tx,
            folio_id,
        )
        .await
        .map_err(|_| {
            ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal server error",
            )
        })?;

    tx.rollback()
        .await
        .map_err(|_| {
            ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal server error",
            )
        })?;

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
            .collect();

    Ok(Json(response))
}

pub async fn assign_billing_account(
    State(state): State<AppState>,
    Path(folio_id): Path<String>,
    Json(request): Json<AssignBillingAccountRequest>,
) -> Result<StatusCode, ApiError> {

    let folio_id =
        Uuid::parse_str(&folio_id)
            .map_err(|e| {
                map_app_error(
                    AppError::Validation(
                        e.to_string()
                    )
                )
            })?;

    let billing_account_id =
        Uuid::parse_str(
            &request.billing_account_id
        )
        .map_err(|e| {
            map_app_error(
                AppError::Validation(
                    e.to_string()
                )
            )
        })?;

    let mut tx =
        state.db.begin_tx().await;

    let mut folio =
        SqliteFolioRepository::find_by_id(
            &mut tx,
            folio_id,
        )
        .await
        .map_err(|e| {
            map_app_error(
                AppError::Validation(e)
            )
        })?
        .ok_or_else(|| {
            ApiError::new(
                StatusCode::NOT_FOUND,
                "folio not found",
            )
        })?;

    folio.assign_billing_account(
        billing_account_id
    )
    .map_err(|e| {
        map_app_error(
            AppError::Validation(e)
        )
    })?;

    SqliteFolioRepository::save(
        &mut tx,
        &folio,
    )
    .await
    .map_err(|e| {
        map_app_error(
            AppError::Validation(e)
        )
    })?;

    tx.commit()
        .await
        .map_err(|e| {
            map_app_error(
                AppError::Validation(
                    e.to_string()
                )
            )
        })?;

    Ok(StatusCode::OK)
}

pub async fn issue_invoice_handler(
    State(state): State<AppState>,
    Json(request): Json<IssueInvoiceInput>,
) -> Result<
    Json<IssueInvoiceResponse>,
    ApiError,
> {

    let output =
        issue_invoice(
            &state.db,
            request,
        )
        .await
        .map_err(|e| {
            map_app_error(
                AppError::Validation(e)
            )
        })?;

    Ok(Json(
        IssueInvoiceResponse {

            invoice_id:
                output.invoice_id
                    .to_string(),

            receivable_id:
                output.receivable_id
                    .to_string(),
        }
    ))
}
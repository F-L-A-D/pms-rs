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
            AssignBillingAccountRequest,
            IssueInvoiceRequest,
            IssueInvoiceResponse,
        },

        error::{
            ApiError,
            map_app_error,
        },

        state::AppState,
    },

    usecase::billing::command::{
        assign_billing_account::
            assign_billing_account,

        issue_invoice::
            issue_invoice,
    },
};

pub async fn assign_billing_account_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<AssignBillingAccountRequest>,
) -> Result<
    StatusCode,
    ApiError,
> {

    let folio_id =
        Uuid::parse_str(&id)
            .map_err(|e| {
                ApiError::new(
                    StatusCode::BAD_REQUEST,
                    e.to_string(),
                )
            })?;

    let billing_account_id =
        Uuid::parse_str(
            &request.billing_account_id
        )
        .map_err(|e| {
            ApiError::new(
                StatusCode::BAD_REQUEST,
                e.to_string(),
            )
        })?;

    assign_billing_account(
        &state.db,
        folio_id,
        billing_account_id,
    )
    .await
    .map_err(map_app_error)?;

    Ok(StatusCode::OK)
}

pub async fn issue_invoice_handler(
    State(state): State<AppState>,
    Json(request): Json<IssueInvoiceRequest>,
) -> Result<
    (StatusCode, Json<IssueInvoiceResponse>),
    ApiError,
> {

    let (
        invoice_id,
        receivable_id,
    ) =
        issue_invoice(
            &state.db,
            request.folio_id,
        )
        .await
        .map_err(map_app_error)?;

    Ok((
        StatusCode::CREATED,

        Json(
            IssueInvoiceResponse {
                invoice_id,

                receivable_id,
            }
        )
    ))
}
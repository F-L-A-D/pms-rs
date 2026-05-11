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
            AssignBillingAccountRequest,
            IssueInvoiceRequest,
            IssueInvoiceResponse,
        },

        error::{
            ApiError,
            map_app_error,
        },
    },

    api::state::AppState,

    error::app_error::AppError,

    usecase::billing::{

        assign_billing_account::{
            assign_billing_account,
            AssignBillingAccountInput,
        },

        issue_invoice::issue_invoice,
    },
};

pub async fn assign_billing_account_handler(

    State(state): State<AppState>,

    Path(folio_id): Path<String>,

    Json(request): Json<AssignBillingAccountRequest>,

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

    assign_billing_account(
        &state.db,

        AssignBillingAccountInput {
            folio_id,
            billing_account_id,
        },
    )
    .await
    .map_err(map_app_error)?;

    Ok(
        StatusCode::OK
            .into_response()
    )
}

pub async fn issue_invoice_handler(

    State(state): State<AppState>,

    Json(request): Json<IssueInvoiceRequest>,

) -> Result<Response, ApiError> {

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

    Ok(
        Json(
            IssueInvoiceResponse {

                invoice_id,

                receivable_id,
            }
        )
        .into_response()
    )
}
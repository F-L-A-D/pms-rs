use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use uuid::Uuid;

use crate::{
    api::{
        dto::business_date::{
            input::{
                finalize_night_audit_input::FinalizeNightAuditInput,
                start_night_audit_input::StartNightAuditInput,
            },
            request::{
                finalize_night_audit_request::FinalizeNightAuditRequest,
                start_night_audit_request::StartNightAuditRequest,
            },
            response::{
                business_date_response::BusinessDateResponse,
                night_audit_response::{FinalizeNightAuditResponse, StartNightAuditResponse},
                night_audit_worklist_response::{
                    NightAuditRoomChargeCandidateResponse, NightAuditWorklistResponse,
                    PostNightAuditRoomChargesResponse,
                },
            },
        },
        dto::response::reservation::ReservationResponse,
        error::{map_app_error, ApiError},
        handlers::reservation::reservation_to_response,
        state::AppState,
    },
    domain::semantic::operation_context::OperationContext,
    usecase::business_date::{
        command::{
            extend_departure, finalize_night_audit, mark_no_show_arrival, post_room_charges,
            start_night_audit,
        },
        detail::{get_current_business_date, get_night_audit_worklist},
    },
};

pub async fn get_current_business_date_handler(
    State(state): State<AppState>,
) -> Result<Json<BusinessDateResponse>, ApiError> {
    let business_date = get_current_business_date::execute(&state.db)
        .await
        .map_err(map_app_error)?;

    Ok(Json(BusinessDateResponse::from(business_date)))
}

pub async fn start_night_audit_handler(
    State(state): State<AppState>,
    Json(request): Json<StartNightAuditRequest>,
) -> Result<Json<StartNightAuditResponse>, ApiError> {
    let input = StartNightAuditInput {
        reason: request.reason,
    };

    let business_date = start_night_audit::execute(&state.db, input)
        .await
        .map_err(map_app_error)?;

    Ok(Json(StartNightAuditResponse {
        business_date: BusinessDateResponse::from(business_date),
    }))
}

pub async fn get_night_audit_worklist_handler(
    State(state): State<AppState>,
) -> Result<Json<NightAuditWorklistResponse>, ApiError> {
    let worklist = get_night_audit_worklist::execute(&state.db)
        .await
        .map_err(map_app_error)?;

    Ok(Json(NightAuditWorklistResponse::from(worklist)))
}

pub async fn post_night_audit_room_charges_handler(
    State(state): State<AppState>,
) -> Result<Json<PostNightAuditRoomChargesResponse>, ApiError> {
    let result = post_room_charges::execute(&state.db)
        .await
        .map_err(map_app_error)?;

    Ok(Json(PostNightAuditRoomChargesResponse {
        posted_room_charges: result
            .posted_room_charges
            .into_iter()
            .map(NightAuditRoomChargeCandidateResponse::from)
            .collect(),
    }))
}

pub async fn mark_no_show_arrival_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ReservationResponse>, ApiError> {
    let reservation_id =
        Uuid::parse_str(&id).map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    let reservation =
        mark_no_show_arrival::execute(&state.db, reservation_id, OperationContext::api_system())
            .await
            .map_err(map_app_error)?;

    Ok(Json(reservation_to_response(reservation)))
}

pub async fn extend_departure_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ReservationResponse>, ApiError> {
    let reservation_id =
        Uuid::parse_str(&id).map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    let reservation =
        extend_departure::execute(&state.db, reservation_id, OperationContext::api_system())
            .await
            .map_err(map_app_error)?;

    Ok(Json(reservation_to_response(reservation)))
}

pub async fn finalize_night_audit_handler(
    State(state): State<AppState>,
    Json(request): Json<FinalizeNightAuditRequest>,
) -> Result<Json<FinalizeNightAuditResponse>, ApiError> {
    let input = FinalizeNightAuditInput {
        reason: request.reason,
    };

    let result = finalize_night_audit::execute(&state.db, input)
        .await
        .map_err(map_app_error)?;

    Ok(Json(FinalizeNightAuditResponse {
        closed_business_date: BusinessDateResponse::from(result.closed_business_date),
        current_business_date: BusinessDateResponse::from(result.current_business_date),
    }))
}

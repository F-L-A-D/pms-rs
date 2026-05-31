use axum::{extract::State, Json};

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
        error::{map_app_error, ApiError},
        state::AppState,
    },
    usecase::business_date::{
        command::{finalize_night_audit, post_room_charges, start_night_audit},
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

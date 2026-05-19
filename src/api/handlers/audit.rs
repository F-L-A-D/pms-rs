use axum::{
    extract::{Path, State},
    Json,
};

use uuid::Uuid;

use crate::{
    api::{
        dto::audit::OperationalAuditLogResponse,
        error::{map_app_error, ApiError},
        state::AppState,
    },
    usecase::audit::search::list_audit_logs,
};

pub async fn list_audit_logs_handler(
    State(state): State<AppState>,
    Path((aggregate_type, aggregate_id)): Path<(String, String)>,
) -> Result<Json<Vec<OperationalAuditLogResponse>>, ApiError> {
    let aggregate_id = Uuid::parse_str(&aggregate_id)
        .map_err(|e| ApiError::new(axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;

    let logs = list_audit_logs::execute(&state.db, aggregate_type, aggregate_id)
        .await
        .map_err(map_app_error)?;

    Ok(Json(logs.into_iter().map(Into::into).collect()))
}

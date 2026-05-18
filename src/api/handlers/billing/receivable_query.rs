use axum::{
    extract::{Query, State},
    Json,
};

use chrono::{NaiveDate, Utc};

use serde::Deserialize;

use crate::{
    api::{
        dto::billing::response::receivable_aging_response::ReceivableAgingResponse,
        error::{map_app_error, ApiError},
        state::AppState,
    },
    usecase::billing::search::list_receivable_aging,
};

#[derive(Debug, Deserialize)]
pub struct ReceivableAgingQuery {
    pub as_of_date: Option<NaiveDate>,
}

pub async fn get_receivable_aging_handler(
    State(state): State<AppState>,
    Query(query): Query<ReceivableAgingQuery>,
) -> Result<Json<ReceivableAgingResponse>, ApiError> {
    let as_of_date = query.as_of_date.unwrap_or_else(|| Utc::now().date_naive());

    let response = list_receivable_aging::execute(&state.db, as_of_date)
        .await
        .map_err(map_app_error)?;

    Ok(Json(response))
}

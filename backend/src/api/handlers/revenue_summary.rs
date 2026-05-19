use axum::{
    extract::{Query, State},
    Json,
};

use chrono::NaiveDate;

use serde::Deserialize;

use crate::{
    api::{
        dto::revenue_summary::RevenueSummaryLineResponse,
        error::{map_app_error, ApiError},
        state::AppState,
    },
    error::app_error::validation,
    usecase::revenue_summary::search::{summarize_daily_revenue, summarize_monthly_revenue},
};

#[derive(Debug, Deserialize)]
pub struct DailyRevenueSummaryQuery {
    pub date: NaiveDate,
}

#[derive(Debug, Deserialize)]
pub struct MonthlyRevenueSummaryQuery {
    pub year_month: String,
}

pub async fn get_daily_revenue_summary_handler(
    State(state): State<AppState>,
    Query(query): Query<DailyRevenueSummaryQuery>,
) -> Result<Json<Vec<RevenueSummaryLineResponse>>, ApiError> {
    let lines = summarize_daily_revenue::execute(&state.db, query.date)
        .await
        .map_err(map_app_error)?;

    Ok(Json(
        lines
            .into_iter()
            .map(RevenueSummaryLineResponse::from)
            .collect(),
    ))
}

pub async fn get_monthly_revenue_summary_handler(
    State(state): State<AppState>,
    Query(query): Query<MonthlyRevenueSummaryQuery>,
) -> Result<Json<Vec<RevenueSummaryLineResponse>>, ApiError> {
    if query.year_month.len() != 7 {
        return Err(map_app_error(validation("year_month must be YYYY-MM")));
    }

    let lines = summarize_monthly_revenue::execute(&state.db, query.year_month)
        .await
        .map_err(map_app_error)?;

    Ok(Json(
        lines
            .into_iter()
            .map(RevenueSummaryLineResponse::from)
            .collect(),
    ))
}

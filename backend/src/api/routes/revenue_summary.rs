use axum::{
    routing::get,
    Router,
};

use crate::api::{
    handlers::revenue_summary::{
        get_daily_revenue_summary_handler,
        get_monthly_revenue_summary_handler,
    },
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/revenue-summary/daily",
            get(get_daily_revenue_summary_handler),
        )
        .route(
            "/revenue-summary/monthly",
            get(get_monthly_revenue_summary_handler),
        )
}
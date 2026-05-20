use axum::{
    routing::get,
    Router,
};

use crate::api::{
    handlers::audit::list_audit_logs_handler,
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/audit-logs/:aggregate_type/:aggregate_id",
            get(list_audit_logs_handler),
        )
}
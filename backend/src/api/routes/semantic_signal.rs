use axum::{routing::get, Router};

use crate::api::{
    handlers::semantic_signal::get_operation_semantic_signal_handler, state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new().route(
        "/operation-events/:id/semantic-signal",
        get(get_operation_semantic_signal_handler),
    )
}

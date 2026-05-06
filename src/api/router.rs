use axum::{
    routing::get,
    Router,
};

use crate::api::handlers::health::health;
use crate::api::state::AppState;

pub fn create_router(
    state: AppState,
) -> Router {

    Router::new()
        .route("/health", get(health))
        .with_state(state)
}
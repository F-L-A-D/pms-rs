use axum::{
    routing::get,
    Router,
};

use crate::api::{
    handlers::health::health,
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
}
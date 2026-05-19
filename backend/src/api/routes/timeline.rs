use axum::{
    routing::get,
    Router,
};

use crate::api::{
    handlers::timeline::get_guest_timelines_handler,
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/guests/:id/timelines", 
            get(get_guest_timelines_handler),
        )
}
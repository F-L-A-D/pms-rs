use axum::{
    routing::post,
    Router,
};

use crate::api::{
    handlers::housekeeping::{
        finish_cleaning_handler,
        inspect_room_handler,
        mark_dirty_handler,
        start_cleaning_handler,
    },
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/housekeeping/:id/dirty",
            post(mark_dirty_handler),
        )
        .route(
            "/housekeeping/:id/start-cleaning",
            post(start_cleaning_handler),
        )
        .route(
            "/housekeeping/:id/finish-cleaning",
            post(finish_cleaning_handler),
        )
        .route(
            "/housekeeping/:id/inspect", 
            post(inspect_room_handler),
        )
}
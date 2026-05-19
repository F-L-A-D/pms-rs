use axum::{
    routing::post,
    Router,
};

use crate::api::{
    handlers::stay::{
        assign_room_handler,
        check_in_handler,
        check_out_handler,
        move_room_handler,
    },
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/reservations/:id/assign-room/:room_id",
            post(assign_room_handler),
        )
        .route(
            "/reservations/:id/check-in", 
            post(check_in_handler),
        )
        .route(
            "/reservations/:id/check-out", 
            post(check_out_handler),
        )
        .route(
            "/reservations/:id/room-move/:room_id",
            post(move_room_handler),
        )
}
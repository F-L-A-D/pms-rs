use axum::{
    routing::{get, patch, post},
    Router,
};

use crate::api::{
    handlers::room::{
        create_room_handler, get_room_handler, list_rooms_handler, mark_room_out_of_order_handler,
        return_room_to_service_handler, update_room_activation_handler, update_room_handler,
    },
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/rooms", post(create_room_handler).get(list_rooms_handler))
        .route(
            "/rooms/:id",
            get(get_room_handler).patch(update_room_handler),
        )
        .route(
            "/rooms/:id/activation",
            patch(update_room_activation_handler),
        )
        .route(
            "/rooms/:id/out-of-order",
            post(mark_room_out_of_order_handler),
        )
        .route(
            "/rooms/:id/return-to-service",
            post(return_room_to_service_handler),
        )
}

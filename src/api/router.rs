use axum::{
    routing::{
        get,
        post,
        patch,
        delete,
    },
    Router,
};

use crate::api::handlers::health::health;

use crate::api::handlers::stay::{
    assign_room_handler,
    check_in_handler,
    check_out_handler,
};

use crate::api::handlers::room::{
    create_room,
    list_rooms,
};

use crate::api::handlers::reservation::{
    create_reservation,
    modify_reservation,
    cancel_reservation,
};

use crate::api::state::AppState;

pub fn create_router(
    state: AppState,
) -> Router {

    Router::new()

        .route(
            "/health",
            get(health),
        )

        .route(
            "/reservations",
            post(create_reservation),
        )

        .route(
            "/reservations/:id",
            patch(modify_reservation),
        )

        .route(
            "/reservations/:id",
            delete(cancel_reservation),
        )

        .route(
            "/stays/:id/assign-room",
            post(assign_room_handler),
        )

        .route(
            "/stays/:id/check-in",
            post(check_in_handler),
        )

        .route(
            "/stays/:id/check-out",
            post(check_out_handler),
        )

        .route(
            "/rooms",
            post(create_room),
        )

        .route(
            "/rooms",
            get(list_rooms),
        )

        .with_state(state)
}
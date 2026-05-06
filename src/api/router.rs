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

        .with_state(state)
}
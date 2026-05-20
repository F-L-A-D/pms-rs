use axum::{
    routing::{get, post},
    Router,
};

use crate::api::{
    handlers::guest::{
        add_guest_preference_handler,
        create_guest_handler,
        get_guest_handler,
        get_guest_preferences_handler,
        update_guest_handler,
    },
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/guests", post(create_guest_handler))
        .route(
            "/guests/:id",
            get(get_guest_handler)
                .patch(update_guest_handler),
        )
        .route(
            "/guests/:id/preferences",
            post(add_guest_preference_handler)
                .get(get_guest_preferences_handler),
        )
}
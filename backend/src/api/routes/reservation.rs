use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use crate::api::{
    handlers::reservation::{
        cancel_reservation_handler,
        close_reservation_edit_session_handler,
        create_reservation_handler,
        create_reservation_note_handler,
        create_reservation_trace_handler,
        delete_reservation_note_handler,
        delete_reservation_trace_handler,
        get_guest_reservations_handler,
        get_reservation_handler,
        mark_no_show_handler,
        modify_reservation_handler,
        open_reservation_edit_session_handler,
        reinstate_reservation_handler,
        resolve_reservation_trace_handler,
    },
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/reservations", post(create_reservation_handler))
        .route(
            "/reservations/:id",
            patch(modify_reservation_handler)
                .delete(cancel_reservation_handler)
                .get(get_reservation_handler),
        )
        .route(
            "/guests/:id/reservations",
            get(get_guest_reservations_handler),
        )
        .route("/reservations/:id/no-show", post(mark_no_show_handler))
        .route(
            "/reservations/:id/reinstate",
            post(reinstate_reservation_handler),
        )
        .route(
            "/reservations/:id/edit-sessions",
            post(open_reservation_edit_session_handler),
        )
        .route(
            "/reservation-edit-sessions/:id",
            delete(close_reservation_edit_session_handler),
        )
        .route(
            "/reservations/:id/notes",
            post(create_reservation_note_handler),
        )
        .route(
            "/reservations/:id/notes/:note_id",
            delete(delete_reservation_note_handler),
        )
        .route(
            "/reservations/:id/traces",
            post(create_reservation_trace_handler),
        )
        .route(
            "/reservations/:id/traces/:trace_id",
            delete(delete_reservation_trace_handler),
        )
        .route(
            "/reservations/:id/traces/:trace_id/resolve",
            post(resolve_reservation_trace_handler),
        )
}
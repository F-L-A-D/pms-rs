#[allow(unused_imports)]
use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use crate::api::handlers::billing::{
    assign_billing_account::assign_billing_account_handler,
    create_folio_entry::create_folio_entry_handler, create_invoice::create_invoice_handler,
    create_payment::create_payment_handler,
};

use crate::api::handlers::guest::{create_guest_handler, get_guest_handler, update_guest_handler};

use crate::api::handlers::health::health;

use crate::api::handlers::housekeeping::{
    finish_cleaning_handler, inspect_room_handler, mark_dirty_handler, start_cleaning_handler,
};

use crate::api::handlers::reservation::{
    cancel_reservation_handler, create_reservation_handler, get_guest_reservations_handler,
    get_reservation_handler, modify_reservation_handler,
};

use crate::api::handlers::room::{
    create_room_handler, get_room_handler, list_rooms_handler, update_room_activation_handler,
    update_room_handler,
};

use crate::api::handlers::semantic_signal::get_operation_semantic_signal_handler;

use crate::api::handlers::stay::{assign_room_handler, check_in_handler, check_out_handler};

use crate::api::handlers::timeline::get_guest_timelines_handler;

use crate::api::state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        // reservation
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
        // stay
        .route(
            "/reservations/:id/assign-room/:room_id",
            post(assign_room_handler),
        )
        .route("/reservations/:id/check-in", post(check_in_handler))
        .route("/reservations/:id/check-out", post(check_out_handler))
        // room
        .route("/rooms", post(create_room_handler).get(list_rooms_handler))
        .route(
            "/rooms/:id",
            get(get_room_handler).patch(update_room_handler),
        )
        .route(
            "/rooms/:id/activation",
            patch(update_room_activation_handler),
        )
        // housekeeping
        .route("/housekeeping/:id/dirty", post(mark_dirty_handler))
        .route(
            "/housekeeping/:id/start-cleaning",
            post(start_cleaning_handler),
        )
        .route(
            "/housekeeping/:id/finish-cleaning",
            post(finish_cleaning_handler),
        )
        .route("/housekeeping/:id/inspect", post(inspect_room_handler))
        // billing
        .route("/folios/entries", post(create_folio_entry_handler))
        .route("/folios/payments", post(create_payment_handler))
        // guest
        .route("/guests", post(create_guest_handler))
        .route(
            "/guests/:id",
            get(get_guest_handler).patch(update_guest_handler),
        )
        // timeline
        .route("/guests/:id/timelines", get(get_guest_timelines_handler))
        // semantic signal
        .route(
            "/operation-events/:id/semantic-signal",
            get(get_operation_semantic_signal_handler),
        )
        // invoice
        .route(
            "/folios/:id/billing-account",
            post(assign_billing_account_handler),
        )
        .route("/invoices", post(create_invoice_handler))
        .with_state(state)
}

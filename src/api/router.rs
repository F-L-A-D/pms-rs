use axum::{
    routing::{
        delete,
        get,
        patch,
        post,
    },
    Router,
};

use crate::api::handlers::billing::{
    get_balance_handler,
    get_entries_handler,
    open_folio_handler,
    post_payment_handler,
    post_room_charge_handler,
};

use crate::api::handlers::guest::{
    create_guest_handler,
    get_guest_handler,
    list_guests_handler,
    update_guest_handler,
};

use crate::api::handlers::health::health;

use crate::api::handlers::housekeeping::{
    finish_cleaning_handler,
    inspect_room_handler,
    mark_dirty_handler,
    start_cleaning_handler,
};

use crate::api::handlers::reservation::{
    cancel_reservation,
    create_reservation,
    modify_reservation,
};

use crate::api::handlers::room::{
    create_room_handler,
    list_rooms_handler,
};

use crate::api::handlers::stay::{
    assign_room_handler,
    check_in_handler,
    check_out_handler,
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

        // reservation

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

        // stay

        .route(
            "/stays/:id/assign-room/:room_id",
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

        // room

        .route(
            "/rooms",
            post(create_room_handler),
        )

        .route(
            "/rooms",
            get(list_rooms_handler),
        )

        // housekeeping

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

        // billing

        .route(
            "/folios",
            post(open_folio_handler),
        )

        .route(
            "/folios/:id/charges",
            post(post_room_charge_handler),
        )

        .route(
            "/folios/:id/payments",
            post(post_payment_handler),
        )

        .route(
            "/folios/:id/balance",
            get(get_balance_handler),
        )

        .route(
            "/folios/:id/entries",
            get(get_entries_handler),
        )

        // guest

        .route(
            "/guests",
            post(create_guest_handler),
        )

        .route(
            "/guests",
            get(list_guests_handler),
        )

        .route(
            "/guests/:id",
            get(get_guest_handler),
        )

        .route(
            "/guests/:id",
            patch(update_guest_handler),
        )

        .with_state(state)
}
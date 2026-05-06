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

use crate::api::handlers::billing::{
    open_folio,
    post_room_charge,
    post_payment,
    get_balance,
    get_entries,
};

use crate::api::handlers::guest::{
    create_guest_handler,
    get_guest_handler,
    list_guests_handler,
    update_guest_handler,
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

        .route(
            "/folios",
            post(open_folio),
        )

        .route(
            "/folios/:id/charges/room",
            post(post_room_charge),
        )

        .route(
            "/folios/:id/payments",
            post(post_payment),
        )

        .route(
            "/folios/:id/balance",
            get(get_balance),
        )

        .route(
            "/folios/:id/entries",
            get(get_entries),
        )

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
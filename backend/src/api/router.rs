#[allow(unused_imports)]
use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use tower_http::cors::{Any, CorsLayer};

use crate::api::handlers::billing::{
    allocate_receivable_payment::allocate_receivable_payment_handler,
    assign_billing_account::assign_billing_account_handler,
    create_deposit::create_deposit_handler,
    create_folio_entry::create_folio_entry_handler,
    create_invoice::create_invoice_handler,
    create_payment::create_payment_handler,
    folio_query::get_folio_handler,
    invoice_query::{get_invoice_handler, list_billing_account_invoices_handler},
    open_reservation_folio::open_reservation_folio_handler,
    receivable_detail::get_receivable_handler,
    receivable_query::get_receivable_aging_handler,
    reverse_payment_allocation::reverse_payment_allocation_handler,
    update_receivable_status::{
        dispute_receivable_handler, resolve_receivable_dispute_handler,
        write_off_receivable_handler,
    },
    void_invoice::void_invoice_handler,
};

use crate::api::handlers::audit::list_audit_logs_handler;

use crate::api::handlers::guest::{
    add_guest_preference_handler, create_guest_handler, get_guest_handler,
    get_guest_preferences_handler, update_guest_handler,
};

use crate::api::handlers::health::health;

use crate::api::handlers::housekeeping::{
    finish_cleaning_handler, inspect_room_handler, mark_dirty_handler, start_cleaning_handler,
};

use crate::api::handlers::package::{
    assign_package_to_plan_handler, create_package_definition_handler, create_rate_plan_handler,
    get_package_definition_handler, get_rate_plan_handler, list_plan_packages_handler,
    update_package_activation_handler, update_package_definition_handler,
    update_rate_plan_activation_handler,
};

use crate::api::handlers::reservation::{
    cancel_reservation_handler, close_reservation_edit_session_handler, create_reservation_handler,
    create_reservation_note_handler, get_guest_reservations_handler, get_reservation_handler,
    mark_no_show_handler, modify_reservation_handler, open_reservation_edit_session_handler,
    reinstate_reservation_handler,
};

use crate::api::handlers::revenue_summary::{
    get_daily_revenue_summary_handler, get_monthly_revenue_summary_handler,
};

use crate::api::handlers::room::{
    create_room_handler, get_room_handler, list_rooms_handler, mark_room_out_of_order_handler,
    return_room_to_service_handler, update_room_activation_handler, update_room_handler,
};

use crate::api::handlers::semantic_signal::get_operation_semantic_signal_handler;

use crate::api::handlers::stay::{
    assign_room_handler, check_in_handler, check_out_handler, move_room_handler,
};

use crate::api::handlers::timeline::get_guest_timelines_handler;

use crate::api::state::AppState;

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/health", get(health))
        // audit
        .route(
            "/audit-logs/:aggregate_type/:aggregate_id",
            get(list_audit_logs_handler),
        )
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
            "/reservations/:id/notes",
            post(create_reservation_note_handler),
        )
        .route(
            "/reservation-edit-sessions/:id",
            delete(close_reservation_edit_session_handler),
        )
        .route(
            "/reservations/:id/folios",
            post(open_reservation_folio_handler),
        )
        // stay
        .route(
            "/reservations/:id/assign-room/:room_id",
            post(assign_room_handler),
        )
        .route("/reservations/:id/check-in", post(check_in_handler))
        .route("/reservations/:id/check-out", post(check_out_handler))
        .route(
            "/reservations/:id/room-move/:room_id",
            post(move_room_handler),
        )
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
        .route(
            "/rooms/:id/out-of-order",
            post(mark_room_out_of_order_handler),
        )
        .route(
            "/rooms/:id/return-to-service",
            post(return_room_to_service_handler),
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
        // package / rate plan catalog
        .route("/packages", post(create_package_definition_handler))
        .route(
            "/packages/:package_code",
            get(get_package_definition_handler).patch(update_package_definition_handler),
        )
        .route(
            "/packages/:package_code/activation",
            patch(update_package_activation_handler),
        )
        .route("/rate-plans", post(create_rate_plan_handler))
        .route("/rate-plans/:plan_code", get(get_rate_plan_handler))
        .route(
            "/rate-plans/:plan_code/activation",
            patch(update_rate_plan_activation_handler),
        )
        .route(
            "/rate-plans/:plan_code/packages",
            post(assign_package_to_plan_handler).get(list_plan_packages_handler),
        )
        // revenue summary
        .route(
            "/revenue-summary/daily",
            get(get_daily_revenue_summary_handler),
        )
        .route(
            "/revenue-summary/monthly",
            get(get_monthly_revenue_summary_handler),
        )
        // billing
        .route("/folios/entries", post(create_folio_entry_handler))
        .route("/folios/payments", post(create_payment_handler))
        .route("/folios/deposits", post(create_deposit_handler))
        .route("/folios/:id", get(get_folio_handler))
        .route("/receivables/aging", get(get_receivable_aging_handler))
        .route("/receivables/:id", get(get_receivable_handler))
        .route(
            "/receivables/:id/payments",
            post(allocate_receivable_payment_handler),
        )
        .route("/receivables/:id/dispute", post(dispute_receivable_handler))
        .route(
            "/receivables/:id/resolve-dispute",
            post(resolve_receivable_dispute_handler),
        )
        .route(
            "/receivables/:id/write-off",
            post(write_off_receivable_handler),
        )
        .route(
            "/payment-allocations/:id/reverse",
            post(reverse_payment_allocation_handler),
        )
        // guest
        .route("/guests", post(create_guest_handler))
        .route(
            "/guests/:id",
            get(get_guest_handler).patch(update_guest_handler),
        )
        .route(
            "/guests/:id/preferences",
            post(add_guest_preference_handler).get(get_guest_preferences_handler),
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
        .route("/invoices/:id", get(get_invoice_handler))
        .route("/invoices/:id/void", post(void_invoice_handler))
        .route(
            "/billing-accounts/:id/invoices",
            get(list_billing_account_invoices_handler),
        )
        .with_state(state)
        .layer(cors)
}

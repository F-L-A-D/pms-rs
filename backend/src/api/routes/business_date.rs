use axum::{
    routing::{get, post},
    Router,
};

use crate::api::{
    handlers::business_date::{
        finalize_night_audit_handler, get_current_business_date_handler,
        get_night_audit_worklist_handler, mark_no_show_arrival_handler,
        post_night_audit_room_charges_handler, start_night_audit_handler,
    },
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/business-date/current",
            get(get_current_business_date_handler),
        )
        .route(
            "/business-date/night-audit/start",
            post(start_night_audit_handler),
        )
        .route(
            "/business-date/night-audit/worklist",
            get(get_night_audit_worklist_handler),
        )
        .route(
            "/business-date/night-audit/post-room-charges",
            post(post_night_audit_room_charges_handler),
        )
        .route(
            "/business-date/night-audit/arrivals/:id/no-show",
            post(mark_no_show_arrival_handler),
        )
        .route(
            "/business-date/night-audit/finalize",
            post(finalize_night_audit_handler),
        )
}

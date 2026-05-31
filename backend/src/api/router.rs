use axum::Router;

use tower_http::cors::{Any, CorsLayer};

use crate::api::{routes, state::AppState};

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .merge(routes::health::routes())
        .merge(routes::audit::routes())
        .merge(routes::business_date::routes())
        .merge(routes::reservation::routes())
        .merge(routes::stay::routes())
        .merge(routes::room::routes())
        .merge(routes::housekeeping::routes())
        .merge(routes::package::routes())
        .merge(routes::revenue_summary::routes())
        .merge(routes::billing::routes())
        .merge(routes::guest::routes())
        .merge(routes::timeline::routes())
        .merge(routes::semantic_signal::routes())
        .with_state(state)
        .layer(cors)
}

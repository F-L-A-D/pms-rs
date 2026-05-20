use axum::{
    routing::{get, patch, post},
    Router,
};

use crate::api::{
    handlers::package::{
        assign_package_to_plan_handler, create_package_definition_handler,
        create_rate_plan_handler, get_package_definition_handler, get_rate_plan_handler,
        list_plan_packages_handler, update_package_activation_handler,
        update_package_definition_handler, update_rate_plan_activation_handler,
    },
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
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
}

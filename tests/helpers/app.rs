use axum::Router;

use pms_rs::api::{
    router::create_router,
    state::AppState,
};

use pms_rs::db::connection::Db;

pub async fn test_app() -> Router {

    let db =
        Db::new("sqlite::memory:")
            .await;

    let state =
        AppState {
            db,
        };

    create_router(state)
}
use axum::Router;

use pms_rs::api::{
    router::create_router,
    state::AppState,
};

use pms_rs::db::connection::Db;

pub struct TestApp {
    pub app: Router,
    pub db: Db,
}

pub async fn test_app() -> TestApp {

    let db =
        Db::new("sqlite::memory:")
            .await;

    let state =
        AppState {
            db: db.clone(),
        };

    let app =
        create_router(state);

    TestApp {
        app,
        db,
    }
}
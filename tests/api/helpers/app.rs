use axum::Router;

use pms_rs::{
    api::{
        router::create_router,
        state::AppState,
    },
    db::connection::Db,
};

pub struct TestApp {
    pub app: Router,
    pub db: Db,
}

pub async fn spawn_app() -> TestApp {

    let db = Db::new_test().await;

    let state = AppState {
        db: db.clone(),
    };

    let app = create_router(state);

    TestApp {
        app,
        db,
    }
}
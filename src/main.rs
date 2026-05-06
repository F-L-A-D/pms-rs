use tokio::net::TcpListener;

use pms_rs::db::connection::Db;

use pms_rs::api::router::create_router;
use pms_rs::api::state::AppState;

#[tokio::main]
async fn main() {

    let db =
        Db::new("sqlite:data/pms.db")
            .await;

    let state =
        AppState { db };

    let app =
        create_router(state);

    let listener =
        TcpListener::bind("0.0.0.0:3000")
            .await
            .unwrap();

    println!("server started on :3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}
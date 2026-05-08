#![allow(unused_imports)]
use pms_rs::db::connection::Db;
use pms_rs::repository::sqlite::inventory_repository::SqliteInventoryRepository;

#[tokio::test]
async fn inventory_should_not_go_negative() {
    let db = Db::new("sqlite::memory:").await;

    let mut tx = db.begin_tx().await;

    let result = SqliteInventoryRepository::add_tx(
        &mut tx,
        "2026-05-01",
        -1,
        10,
    )
    .await;

    assert!(result.is_err());
}
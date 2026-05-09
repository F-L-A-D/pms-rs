use sqlx::SqlitePool;

pub mod behavioral;
pub mod operational;
pub mod projection;

pub async fn bootstrap(
    pool: &SqlitePool,
) {

    operational::bootstrap::bootstrap(pool)
        .await;

    behavioral::bootstrap::bootstrap(pool)
        .await;

    projection::bootstrap::bootstrap(pool)
        .await;
}
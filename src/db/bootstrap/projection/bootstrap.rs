use sqlx::SqlitePool;

pub async fn bootstrap(
    pool: &SqlitePool,
) {

    sqlx::query(
        include_str!(
            "guest_summary_projections.sql"
        ),
    )
    .execute(pool)
    .await
    .unwrap();
}
use sqlx::SqlitePool;

pub async fn bootstrap(
    pool: &SqlitePool,
) {

    sqlx::query(
        include_str!(
            "guest_aggregates.sql"
        ),
    )
    .execute(pool)
    .await
    .unwrap();

}
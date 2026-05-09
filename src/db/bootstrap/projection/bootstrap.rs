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

    sqlx::query(
        include_str!(
            "reservation_search_projections.sql"
        ),
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        include_str!(
            "inventory_projection.sql"
        ),
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        include_str!(
            "hotel_inventory_projection.sql"
        ),
    )
    .execute(pool)
    .await
    .unwrap();
}
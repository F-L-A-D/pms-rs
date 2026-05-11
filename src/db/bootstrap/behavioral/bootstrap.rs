use sqlx::SqlitePool;

pub async fn bootstrap(
    pool: &SqlitePool,
) {

    sqlx::query(
        include_str!("folio_entries.sql"),
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        include_str!(
            "guest_timeline_events.sql"
        ),
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        include_str!(
            "settlement_transitions.sql"
        ),
    )
    .execute(pool)
    .await
    .unwrap();
}
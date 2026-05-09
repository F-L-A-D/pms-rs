use sqlx::SqlitePool;

pub async fn bootstrap(
    pool: &SqlitePool,
) {

    sqlx::query(
        include_str!("reservations.sql"),
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        include_str!("inventory.sql"),
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        include_str!("rooms.sql"),
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        include_str!("folios.sql"),
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        include_str!("guests.sql"),
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        include_str!(
            "reservation_guest_relations.sql"
        ),
    )
    .execute(pool)
    .await
    .unwrap();
}
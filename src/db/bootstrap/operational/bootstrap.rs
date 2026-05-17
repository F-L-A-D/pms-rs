use sqlx::SqlitePool;

pub async fn bootstrap(pool: &SqlitePool) {
    sqlx::query(include_str!("reservations.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("rooms.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("room_daily_states.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("folios.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("guests.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("reservation_guest_relations.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("reservation_package_breakdowns.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("companies.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("billing_accounts.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("invoices.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("receivables.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("payments.sql"))
        .execute(pool)
        .await
        .unwrap();
}

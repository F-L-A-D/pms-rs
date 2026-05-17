use sqlx::SqlitePool;

pub async fn bootstrap(pool: &SqlitePool) {
    sqlx::query(include_str!("guest_aggregates.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("guest_activities.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("housekeeping_daily_workload_aggregates.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("inventory_aggregates.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("daily_room_class_kpi_aggregates.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("daily_hotel_kpi_aggregates.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("monthly_room_class_kpi_aggregates.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("monthly_hotel_kpi_aggregates.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("change_patterns.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("confidence_profiles.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("semantic_activations.sql"))
        .execute(pool)
        .await
        .unwrap();
}

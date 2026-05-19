use sqlx::SqlitePool;

pub async fn bootstrap(pool: &SqlitePool) {
    sqlx::query(include_str!("reservation/reservations.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("reservation/reservation_edit_sessions.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("room/rooms.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("room/room_daily_states.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("billing/folios.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("guest/guests.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("guest/guest_preferences.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("reservation/reservation_guest_relations.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!(
        "reservation/reservation_package_breakdowns.sql"
    ))
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(include_str!(
        "reservation/reservation_daily_stay_details.sql"
    ))
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(include_str!(
        "reservation/reservation_daily_revenue_allocations.sql"
    ))
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(include_str!("rate_plan/package_definitions.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("rate_plan/rate_plan_definitions.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("rate_plan/rate_plan_packages.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("operation/operation_change_events.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("operation/operational_audit_logs.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("billing/companies.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("billing/billing_accounts.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("billing/invoices.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_invoices_invoice_number_unique ON invoices(invoice_number)",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(include_str!("billing/receivables.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("billing/payments.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("billing/payment_allocations.sql"))
        .execute(pool)
        .await
        .unwrap();
}

use sqlx::{Row, SqlitePool};

pub async fn bootstrap(pool: &SqlitePool) {
    sqlx::query(include_str!("reservation/reservations.sql"))
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

    ensure_reservation_daily_revenue_allocation_snapshot_columns(pool).await;

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

    ensure_invoice_due_date_column(pool).await;

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

    ensure_receivable_due_date_column(pool).await;

    sqlx::query(include_str!("billing/payments.sql"))
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(include_str!("billing/payment_allocations.sql"))
        .execute(pool)
        .await
        .unwrap();

    ensure_payment_allocation_reversed_at_column(pool).await;
}

async fn ensure_invoice_due_date_column(pool: &SqlitePool) {
    if has_column(pool, "invoices", "due_date").await {
        return;
    }

    sqlx::query("ALTER TABLE invoices ADD COLUMN due_date TEXT NOT NULL DEFAULT '1970-01-01'")
        .execute(pool)
        .await
        .unwrap();
}

async fn ensure_receivable_due_date_column(pool: &SqlitePool) {
    if has_column(pool, "receivables", "due_date").await {
        return;
    }

    sqlx::query("ALTER TABLE receivables ADD COLUMN due_date TEXT NOT NULL DEFAULT '1970-01-01'")
        .execute(pool)
        .await
        .unwrap();
}

async fn ensure_payment_allocation_reversed_at_column(pool: &SqlitePool) {
    if has_column(pool, "payment_allocations", "reversed_at").await {
        return;
    }

    sqlx::query("ALTER TABLE payment_allocations ADD COLUMN reversed_at TEXT")
        .execute(pool)
        .await
        .unwrap();
}

async fn ensure_reservation_daily_revenue_allocation_snapshot_columns(pool: &SqlitePool) {
    if !has_column(
        pool,
        "reservation_daily_revenue_allocations",
        "department_code",
    )
    .await
    {
        sqlx::query(
            "ALTER TABLE reservation_daily_revenue_allocations ADD COLUMN department_code TEXT",
        )
        .execute(pool)
        .await
        .unwrap();
    }

    if !has_column(
        pool,
        "reservation_daily_revenue_allocations",
        "account_code",
    )
    .await
    {
        sqlx::query(
            "ALTER TABLE reservation_daily_revenue_allocations ADD COLUMN account_code TEXT",
        )
        .execute(pool)
        .await
        .unwrap();
    }
}

async fn has_column(pool: &SqlitePool, table_name: &str, column_name: &str) -> bool {
    let rows = sqlx::query(&format!("PRAGMA table_info({table_name})"))
        .fetch_all(pool)
        .await
        .unwrap();

    rows.iter()
        .any(|row| row.get::<String, _>("name") == column_name)
}

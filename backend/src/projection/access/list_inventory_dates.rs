use chrono::NaiveDate;

use sqlx::{Row, Sqlite, Transaction};

use crate::error::app_error::{infra, AppResult};

pub async fn list_inventory_dates(tx: &mut Transaction<'_, Sqlite>) -> AppResult<Vec<NaiveDate>> {
    let rows = sqlx::query(
        r#"
        WITH RECURSIVE reservation_dates(service_date, check_out) AS (
            SELECT check_in, check_out
            FROM reservations
            WHERE check_in < check_out

            UNION ALL

            SELECT date(service_date, '+1 day'), check_out
            FROM reservation_dates
            WHERE date(service_date, '+1 day') < check_out
        ),
        inventory_dates AS (
            SELECT service_date FROM reservation_dates

            UNION

            SELECT service_date FROM room_daily_states
        )
        SELECT DISTINCT service_date
        FROM inventory_dates
        ORDER BY service_date
        "#,
    )
    .fetch_all(&mut **tx)
    .await
    .map_err(infra)?;

    rows.iter()
        .map(|row| {
            row.get::<String, _>("service_date")
                .parse::<NaiveDate>()
                .map_err(infra)
        })
        .collect()
}

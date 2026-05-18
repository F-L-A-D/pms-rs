use chrono::NaiveDate;

use sqlx::{Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    error::app_error::{infra, AppResult},
    projection::aggregate::model::guest_aggregate_row::GuestAggregateRow,
};

const QUERY: &str = include_str!("aggregate_guest.sql");

pub async fn aggregate_guest(
    tx: &mut Transaction<'_, Sqlite>,
    guest_id: Uuid,
) -> AppResult<GuestAggregateRow> {
    let row = sqlx::query(QUERY)
        .bind(guest_id.to_string())
        .fetch_one(&mut **tx)
        .await
        .map_err(infra)?;

    Ok(GuestAggregateRow {
        total_stays: row.get("total_stays"),

        total_nights: row.get::<i64, _>("total_nights"),

        total_spending: row.get("total_spending"),

        last_stay_at: row.get::<Option<NaiveDate>, _>("last_stay_at"),
    })
}

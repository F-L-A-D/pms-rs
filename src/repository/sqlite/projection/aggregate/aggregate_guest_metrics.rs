use chrono::NaiveDate;

use sqlx::{
    Row,
    Sqlite,
    Transaction,
};

use uuid::Uuid;

use crate::{
    error::app_error::{
        AppResult,
        infra,
    },

    projection::aggregate::model::
        guest_aggregate_row::GuestAggregateRow,
};

const QUERY: &str =
    include_str!(
        "aggregate_guest_metrics.sql"
    );

pub async fn aggregate_guest_metrics(
    tx: &mut Transaction<'_, Sqlite>,
    guest_id: Uuid,
) -> AppResult<GuestAggregateRow> {

    let row =
        sqlx::query(QUERY)
            .bind(guest_id.to_string())
            .fetch_one(&mut **tx)
            .await
            .map_err(infra)?;

    Ok(
        GuestAggregateRow {
            total_stays:
                row.get("total_stays"),

            total_nights:
                row.get::<f64, _>(
                    "total_nights"
                ) as i64,

            total_spending:
                row.get("total_spending"),

            last_stay_at:
                row.get::<Option<NaiveDate>, _>(
                    "last_stay_at"
                ),
        }
    )
}
use chrono::Utc;

use sqlx::{
    Sqlite,
    Transaction,
};

use uuid::Uuid;

use crate::{
    error::app_error::AppResult,

    projection::aggregate::{
        access::get_guest_aggregate::
            get_guest_aggregate_row,

        model::{
            guest_aggregate::GuestAggregate,
            guest_aggregate_row::GuestAggregateRow,
        },
    },
};

pub async fn materialize_guest_aggregate(
    tx: &mut Transaction<'_, Sqlite>,
    guest_id: Uuid,
) -> AppResult<GuestAggregate> {

    let row: GuestAggregateRow =
        get_guest_aggregate_row(
            tx,
            guest_id,
        )
        .await?;

    Ok(
        GuestAggregate {
            guest_id,

            total_stays:
                row.total_stays,

            total_nights:
                row.total_nights,

            total_spending:
                row.total_spending,

            last_stay_at:
                row.last_stay_at,

            projection_version: 1,

            updated_at:
                Utc::now(),
        }
    )
}
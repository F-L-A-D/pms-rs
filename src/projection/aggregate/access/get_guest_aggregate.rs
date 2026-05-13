use sqlx::{
    Sqlite,
    Transaction,
};

use uuid::Uuid;

use crate::{
    error::app_error::AppResult,

    projection::aggregate::model::
        guest_aggregate_row::GuestAggregateRow,

    repository::sqlite::projection::aggregate::
        aggregate_guest_metrics::aggregate_guest_metrics,
};

pub async fn get_guest_aggregate_row(
    tx: &mut Transaction<'_, Sqlite>,
    guest_id: Uuid,
) -> AppResult<GuestAggregateRow> {

    aggregate_guest_metrics(
        tx,
        guest_id,
    )
    .await
}
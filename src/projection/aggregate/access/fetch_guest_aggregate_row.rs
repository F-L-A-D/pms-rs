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
        aggregate_guest::aggregate_guest,
};

pub async fn fetch_guest_aggregate_row(
    tx: &mut Transaction<'_, Sqlite>,
    guest_id: Uuid,
) -> AppResult<GuestAggregateRow> {

    aggregate_guest(
        tx,
        guest_id,
    )
    .await
}
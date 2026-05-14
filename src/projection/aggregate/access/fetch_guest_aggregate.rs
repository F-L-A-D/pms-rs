use sqlx::{
    Sqlite,
    Transaction,
};

use uuid::Uuid;

use crate::{
    error::app_error::AppResult,

    projection::aggregate::model::
        guest_aggregate::GuestAggregate,

    repository::sqlite::projection::aggregate::
        get_guest_aggregate::get_guest_aggregate,
};

pub async fn fetch_guest_aggregate(
    tx: &mut Transaction<'_, Sqlite>,
    guest_id: Uuid,
) -> AppResult<GuestAggregate> {

    get_guest_aggregate(
        tx,
        guest_id,
    )
    .await
}
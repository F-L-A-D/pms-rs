use sqlx::{
    Sqlite,
    Transaction,
};

use uuid::Uuid;

use crate::{
    error::app_error::AppResult,

    projection::aggregate::{
        materializer::guest_aggregate_materializer::
            materialize_guest_aggregate,

        model::guest_aggregate::GuestAggregate,
    },

    repository::sqlite::projection::save::
        save_guest_aggregate::
            save_guest_aggregate,
};

pub async fn refresh_guest_aggregate(
    tx: &mut Transaction<'_, Sqlite>,
    guest_id: Uuid,
) -> AppResult<GuestAggregate> {

    let aggregate =
        materialize_guest_aggregate(
            tx,
            guest_id,
        )
        .await?;

    save_guest_aggregate(
        tx,
        &aggregate,
    )
    .await?;

    Ok(aggregate)
}
use sqlx::{
    Sqlite,
    Transaction,
};

use crate::{
    error::app_error::AppResult,

    projection::aggregate::{
        materializer::guest_aggregate_materializer::
            materialize_guest_aggregate,
        access::
            list_guest_ids::list_guest_ids
    },

    repository::sqlite::projection::save::
        save_guest_aggregate::
            save_guest_aggregate,
};

pub async fn rebuild_guest_aggregate(
    tx: &mut Transaction<'_, Sqlite>,
) -> AppResult<()> {

    let guest_ids =
        list_guest_ids(tx)
            .await?;

    for guest_id in guest_ids {

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
    }

    Ok(())
}
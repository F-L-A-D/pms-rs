use sqlx::{
    Sqlite,
    Transaction,
};

use crate::{
    error::app_error::AppResult,

    projection::aggregate::rebuild::
        rebuild_guest_aggregate::
            rebuild_guest_aggregate,
};

pub async fn execute_guest_aggregate_rebuild(
    tx: &mut Transaction<'_, Sqlite>,
) -> AppResult<()> {

    rebuild_guest_aggregate(
        tx,
    )
    .await?;

    Ok(())
}
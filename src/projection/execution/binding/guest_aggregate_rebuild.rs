use sqlx::{
    Sqlite,
    Transaction,
};

use crate::{
    error::app_error::AppResult,

    projection::{
        aggregate::rebuild::
            rebuild_guest_aggregate::
                rebuild_guest_aggregate,

        topology::projection_node::ProjectionNode,
    },

    repository::sqlite::projection::
        clear::clear_projection_table::
            clear_projection_table,
};

pub async fn execute_guest_aggregate_rebuild(
    tx: &mut Transaction<'_, Sqlite>,
) -> AppResult<()> {

    clear_projection_table(
        tx,
        ProjectionNode::GuestAggregate,
    )
    .await?;

    rebuild_guest_aggregate(
        tx,
    )
    .await?;

    Ok(())
}
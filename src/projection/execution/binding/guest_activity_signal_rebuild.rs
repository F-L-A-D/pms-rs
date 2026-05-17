use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::{
        signal::rebuild::rebuild_guest_activity_signal::rebuild_guest_activity_signal,
        topology::projection_node::ProjectionNode,
    },
    repository::sqlite::projection::clear::clear_projection_table::clear_projection_table,
};

pub async fn execute_guest_activity_signal_rebuild(
    tx: &mut Transaction<'_, Sqlite>,
) -> AppResult<()> {
    clear_projection_table(tx, ProjectionNode::GuestActivitySignal).await?;

    rebuild_guest_activity_signal(tx).await
}

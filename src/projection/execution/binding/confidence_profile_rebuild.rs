use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::{
        signal::rebuild::rebuild_confidence_profile::rebuild_confidence_profile,
        topology::projection_node::ProjectionNode,
    },
    repository::sqlite::projection::clear::clear_projection_table::clear_projection_table,
};

pub async fn execute_confidence_profile_rebuild(tx: &mut Transaction<'_, Sqlite>) -> AppResult<()> {
    clear_projection_table(tx, ProjectionNode::ConfidenceProfile).await?;

    rebuild_confidence_profile(tx).await
}

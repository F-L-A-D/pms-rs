use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::{
        signal::rebuild::rebuild_change_pattern::rebuild_change_pattern,
        topology::projection_node::ProjectionNode,
    },
    repository::sqlite::projection::clear::clear_projection_table::clear_projection_table,
};

pub async fn execute_change_pattern_rebuild(tx: &mut Transaction<'_, Sqlite>) -> AppResult<()> {
    clear_projection_table(tx, ProjectionNode::ChangePattern).await?;

    rebuild_change_pattern(tx).await
}

use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::{
        signal::rebuild::rebuild_semantic_activation::rebuild_semantic_activation,
        topology::projection_node::ProjectionNode,
    },
    repository::sqlite::projection::clear::clear_projection_table::clear_projection_table,
};

pub async fn execute_semantic_activation_rebuild(
    tx: &mut Transaction<'_, Sqlite>,
) -> AppResult<()> {
    clear_projection_table(tx, ProjectionNode::SemanticActivation).await?;

    rebuild_semantic_activation(tx).await
}

use sqlx::{Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    error::app_error::AppResult,
    projection::{
        execution::execution_trace::push_trace,
        signal::materializer::materialize_semantic_activation::materialize_semantic_activation,
        topology::projection_node::ProjectionNode,
    },
    repository::sqlite::projection::save::save_semantic_activation::save_semantic_activation,
};

pub async fn refresh_semantic_activation(
    tx: &mut Transaction<'_, Sqlite>,
    event_id: Uuid,
) -> AppResult<()> {
    push_trace(ProjectionNode::SemanticActivation);

    let activation = materialize_semantic_activation(tx, event_id).await?;

    save_semantic_activation(tx, &activation).await
}

use sqlx::{Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    error::app_error::AppResult,
    projection::{
        execution::execution_trace::push_trace,
        signal::materializer::materialize_change_pattern::materialize_change_pattern,
        topology::projection_node::ProjectionNode,
    },
    repository::sqlite::projection::save::save_change_pattern::save_change_pattern,
};

pub async fn refresh_change_pattern(
    tx: &mut Transaction<'_, Sqlite>,
    event_id: Uuid,
) -> AppResult<()> {
    push_trace(ProjectionNode::ChangePattern);

    let pattern = materialize_change_pattern(tx, event_id).await?;

    save_change_pattern(tx, &pattern).await
}

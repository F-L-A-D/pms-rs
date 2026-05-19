use sqlx::{Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    error::app_error::AppResult,
    projection::{
        execution::execution_trace::push_trace,
        signal::materializer::materialize_confidence_profile::materialize_confidence_profile,
        topology::projection_node::ProjectionNode,
    },
    repository::sqlite::projection::save::save_confidence_profile::save_confidence_profile,
};

pub async fn refresh_confidence_profile(
    tx: &mut Transaction<'_, Sqlite>,
    event_id: Uuid,
) -> AppResult<()> {
    push_trace(ProjectionNode::ConfidenceProfile);

    let profile = materialize_confidence_profile(tx, event_id).await?;

    save_confidence_profile(tx, &profile).await
}

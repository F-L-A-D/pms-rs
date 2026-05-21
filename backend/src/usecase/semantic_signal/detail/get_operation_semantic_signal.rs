use uuid::Uuid;

use crate::{
    db::connection::Db,
    error::app_error::{infra, not_found, AppResult},
    projection::signal::{
        access::{
            fetch_change_pattern::fetch_change_pattern,
            fetch_confidence_profile::fetch_confidence_profile,
            fetch_semantic_activation::fetch_semantic_activation,
        },
        model::operation_semantic_signal::OperationSemanticSignal,
    },
    repository::sqlite::operational::operation::operation_change_event_repository::SqliteOperationChangeEventRepository,
};

pub async fn execute(db: &Db, event_id: Uuid) -> AppResult<OperationSemanticSignal> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let event = SqliteOperationChangeEventRepository::find_by_id(&mut tx, event_id).await?;

        if event.is_none() {
            return Err(not_found("operation change event not found"));
        }

        Ok(OperationSemanticSignal {
            event_id,
            change_pattern: fetch_change_pattern(&mut tx, event_id).await?,
            confidence_profile: fetch_confidence_profile(&mut tx, event_id).await?,
            semantic_activation: fetch_semantic_activation(&mut tx, event_id).await?,
        })
    }
    .await;

    let _ = tx.rollback().await.map_err(infra);

    result
}

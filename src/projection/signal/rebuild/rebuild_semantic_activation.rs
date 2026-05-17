use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::signal::refresh::refresh_semantic_activation::refresh_semantic_activation,
    repository::sqlite::operational::operation_change_event_repository::SqliteOperationChangeEventRepository,
};

pub async fn rebuild_semantic_activation(tx: &mut Transaction<'_, Sqlite>) -> AppResult<()> {
    for event in SqliteOperationChangeEventRepository::list_all(tx).await? {
        refresh_semantic_activation(tx, event.id).await?;
    }

    Ok(())
}

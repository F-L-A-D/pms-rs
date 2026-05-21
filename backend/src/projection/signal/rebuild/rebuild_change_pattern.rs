use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::signal::refresh::refresh_change_pattern::refresh_change_pattern,
    repository::sqlite::operational::operation::operation_change_event_repository::SqliteOperationChangeEventRepository,
};

pub async fn rebuild_change_pattern(tx: &mut Transaction<'_, Sqlite>) -> AppResult<()> {
    for event in SqliteOperationChangeEventRepository::list_all(tx).await? {
        refresh_change_pattern(tx, event.id).await?;
    }

    Ok(())
}

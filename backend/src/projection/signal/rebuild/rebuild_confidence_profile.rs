use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::signal::refresh::refresh_confidence_profile::refresh_confidence_profile,
    repository::sqlite::operational::operation::operation_change_event_repository::SqliteOperationChangeEventRepository,
};

pub async fn rebuild_confidence_profile(tx: &mut Transaction<'_, Sqlite>) -> AppResult<()> {
    for event in SqliteOperationChangeEventRepository::list_all(tx).await? {
        refresh_confidence_profile(tx, event.id).await?;
    }

    Ok(())
}
